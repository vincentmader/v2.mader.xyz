
use std::collections::HashMap;
use rand::{Rng};
// use std::cmp;

use crate::integrators;
use crate::integrators::Integrator;
use crate::integrators::FieldIntegrator;
use crate::state::Field;
use crate::state::State;
use crate::state::ObjectType;
use crate::interactions::ObjectInteraction;
use crate::interactions::FieldInteraction;


pub struct Engine {

    pub states: Vec<State>,
    page_id: String,
    pub iteration_step: usize,
    pub is_paused: bool,

}
impl Engine {

    pub fn new(page_id: &str) -> Self {
        let states: Vec<State> = Vec::new();
        let page_id = String::from(page_id);
        let is_paused = false;

        Engine { states, page_id, iteration_step: 0, is_paused }
    }

    pub fn init(&mut self) {
        let initial_state = State::new(&self.page_id);
        self.states = Vec::from([initial_state]);
    }

    pub fn step(&mut self, parameters: &HashMap<String, String>) {
        if self.is_paused { return (); }

        let current_state = &self.states[self.iteration_step];
        let mut next_state = current_state.clone();
        next_state.iteration_idx += 1;

        Self::step_objects(&mut next_state, &current_state);
        Self::step_fields(&mut next_state, &current_state);  // TODO 1D, 2D, 3D, bool, spinor...
        self.states.push(next_state);

        self.iteration_step += 1;
    }

    fn step_objects(next_state: &mut State, current_state: &State) {
        for (family_idx, object_family) in next_state.object_families.iter_mut().enumerate() {
            // don't apply forces to statics  // TODO statics on "rails"?
            if matches!(object_family.object_type, ObjectType::Static) { continue }
            // setup integrator
            let integrator = match object_family.integrator {
                Integrator::EulerExplicit => integrators::euler_explicit::step,
                // Integrator::EulerImplicit => integrators::euler_implicit::step,
                // Integrator::RungeKutta2 => integrators::runge_kutta_2::step,
                // Integrator::RungeKutta4 => integrators::runge_kutta_4::step,
                // Integrator::LeapFrog => integrators::leap_frog::step,
                // Integrator::Verlet => integrators::verlet::step,
            };
            // loop over other object families
            for (other_idx, other_family) in current_state.object_families.iter().enumerate() {
                let family_indices = (family_idx, other_idx);
                // don't apply influence of low-mass particles
                if matches!(other_family.object_type, ObjectType::Particle) { continue };
                // choose relevant interactions (both families must "feel" them)  
                let mut interactions: Vec<ObjectInteraction> = Vec::new();
                for object_interaction in object_family.interactions.iter() {
                    for other_interaction in other_family.interactions.iter() {
                        if !matches!(object_interaction, other_interaction) {continue}  // TODO does this work?
                        if !interactions.contains(object_interaction) {
                            interactions.push(object_interaction.clone());
                        }
                    }
                }
                // use integrator to apply interaction -> step object family
                integrator(
                    object_family, other_family, &interactions, family_indices
                );
            }
        }
    }
    fn step_fields(next_state: &mut State, current_state: &State) {

        let neighborhood_type = NeighborhoodType::Moore; // TODO

        let get_neighborhood = match neighborhood_type {
            NeighborhoodType::Neumann => get_neumann_neighborhood,
            NeighborhoodType::Moore => get_moore_neighborhood
        };
        let interactions: Vec<FieldInteraction> = Vec::from([FieldInteraction::Diffusion]);

        for (field_idx, field) in next_state.fields.iter_mut().enumerate() {

            let integrator = match field.integrator {
                FieldIntegrator::Diffusion => integrator_diff,
                FieldIntegrator::Ising => integrator_ising,
                FieldIntegrator::GameOfLife => integrator_game_of_life,
            };

            let mut rng = rand::thread_rng();
            let batch_size = 5000;
            for _ in 0..batch_size {
                let x: f64 = rng.gen();
                let y: f64 = rng.gen();
                let x = x * (*field).dimensions.0 as f64;
                let y = y * (*field).dimensions.1 as f64;
                let x = x as usize;
                let y = y as usize;

                let neighborhood = get_neighborhood(&field, (x, y));
                let mut neighbors: Vec<&Vec<f64>> = Vec::new();
                let mut cell = &mut field.cells[y*field.dimensions.1+x];
                for neighbor_pos in neighborhood.iter() {
                    let col_jdx = neighbor_pos.0;
                    let row_jdx = neighbor_pos.1;
                    let neighbor = &current_state.fields[field_idx].cells[row_jdx*field.dimensions.1+col_jdx];
                    neighbors.push(neighbor);
                }
                integrator(&mut cell, &neighbors, &interactions)
            }
            
            // for row_idx in 0..field.dimensions.1 {
            //     for col_idx in 0..field.dimensions.0 {
            //         let position = (col_idx, row_idx);
            //         let neighborhood = get_neighborhood(&field, position);
            //         let mut cell = &mut field.cells[row_idx*field.dimensions.1+col_idx];
            //         let mut neighbors: Vec<&Vec<f64>> = Vec::new();
            //         for neighbor_pos in neighborhood.iter() {
            //             let col_jdx = neighbor_pos.0;
            //             let row_jdx = neighbor_pos.1;
            //             let neighbor = &current_state.fields[field_idx].cells[row_jdx*field.dimensions.1+col_jdx];
            //             neighbors.push(neighbor);
            //         }
            //         integrator(&mut cell, &neighbors, &interactions)
            //     }
            // }
        }
    }
}

pub fn integrator_game_of_life(
    cell: &mut Vec<f64>, 
    neighbors: &Vec<&Vec<f64>>, 
    interactions: &Vec<FieldInteraction>
) {
    let mut nr_of_living_neighbors = 0;
    for neighbor in neighbors.iter() {
        if neighbor[0] == 1. {
            nr_of_living_neighbors += 1; 
        }
    }
    if cell[0] == 1. {
        if ![2, 3].contains(&nr_of_living_neighbors) {
            cell[0] = -1.;
        }
    } else if cell[0] == -1. {
        if nr_of_living_neighbors == 3 {
            cell[0] = 1.;
        }
    }
}

pub fn integrator_ising(
    cell: &mut Vec<f64>, 
    neighbors: &Vec<&Vec<f64>>, 
    interactions: &Vec<FieldInteraction>
) {
    let (k, J, mu) = (1., 1., 1.); // boltzmann, spin int., magn. moment
    let (B, T) = (0., 1.);

    let spin = cell[0];
    let mut H: f64 = mu * B;

    for neighbor in neighbors.iter() {
        let spjn = neighbor[0];
        H += J * spin * spjn;
    }

    let mut rng = rand::thread_rng();
    let rand: f64 = rng.gen();
    let prob = (-H / (k*T)).exp();
    if rand < prob { (*cell)[0] *= -1. }
}

pub fn integrator_diff(
    cell: &mut Vec<f64>, 
    neighbors: &Vec<&Vec<f64>>, 
    interactions: &Vec<FieldInteraction>
) {
    // interaction
    let mut average_density = 0.;
    for neighbor in neighbors.iter() {
        average_density += neighbor[0];
    }
    average_density /= neighbors.len() as f64;
    // integrator
    let density = &mut cell[0];
    let delta_density = average_density - *density;
    let k = 0.07;
    *density += k * delta_density;  // NOTE: also try with - !
}

pub fn get_moore_neighborhood(field: &Field, position: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighborhood: Vec<(usize, usize)> = Vec::new();
    let x = position.0 as i32;
    let y = position.1 as i32;
    for dx in [-1, 0, 1].iter() {
        if (x + dx) < 0 || (x + dx) >= field.dimensions.0 as i32 { continue; }
        for dy in [-1, 0, 1].iter() {
            if *dx == 0 && *dy == 0 { continue }
            if (y + dy) < 0 || (y + dy) >= field.dimensions.1 as i32 { continue; }
            neighborhood.push(((x+dx) as usize, (y+dy) as usize));
        }
    }
    neighborhood
}
pub fn get_neumann_neighborhood(field: &Field, position: (usize, usize)) -> Vec<(usize, usize)> {
    let neighborhood: Vec<(usize, usize)> = Vec::new();
    // TODO
    neighborhood
}

pub enum NeighborhoodType { Neumann, Moore }
