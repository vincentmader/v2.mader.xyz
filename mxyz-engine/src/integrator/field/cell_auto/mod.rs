#![allow(non_snake_case)]

use rand::Rng;

use crate::state::State;
use crate::state::field::Field;

use crate::interaction::field::field::FieldFieldInteraction;
// use crate::interaction::field::object::FieldObjInteraction;

use crate::config::EngineConfig;
use crate::config::field::FieldEngineConfig;
// use crate::state::field::variant::FieldVariant;
use crate::state::field::relevant_cells::FieldRelevantCells;

use mxyz_physics::thermo_dynamics::boltzmann_prob;
use crate::state::get_cell_idx_from_coords;



pub fn apply_periodic_bounds(idx: i32, dimension: i32) -> i32 {
    if idx < 0                  { idx + dimension }  // 0
    else if idx >= dimension    { idx - dimension }  // dimension - 1
    else                        { idx }
}


pub fn get_flip_energy(
    iter_idx: usize,
    field: &mut Field,
    field_conf: &FieldEngineConfig, 
    states: &Vec<State>,
    x: usize, 
    y: usize,
    z: usize
) -> f64 {
    let (B, J, mu) = (0., 1., 1.);

    let dimensions = &field_conf.dimensions;
    let dim_x = dimensions[0]; // TODO handle 3D
    let dim_y = dimensions[1]; // TODO handle 3D

    let cell_idx = get_cell_idx_from_coords(x, y, z, &field, &field_conf);
    let cell = field.entries[cell_idx]; // TODO generalize to 3D
    let mut dE = 0.;
    for dx in 0..3 {
        for dy in 0..3 {
            // prevent self-interaction
            if (dx == 1) && (dy == 1) { continue; }
            // get coordinates of other cell
            let X = apply_periodic_bounds(x as i32 + dx-1, dim_x as i32) as usize;
            let Y = apply_periodic_bounds(y as i32 + dy-1, dim_y as i32) as usize;
            // get other cell 
            let cell_idx = get_cell_idx_from_coords(X, Y, 0, &field, &field_conf);
            let other = states[iter_idx].fields[field.id].entries[cell_idx];
            // add spin-spin interaction to flip energy
            dE += J*cell*other;
        }
    }
    // add spin-field interaction to flip energy
    dE + mu*cell*B
}


pub fn get_nr_of_neighbors(
    field: &Field, 
    field_conf: &FieldEngineConfig, 
    x: usize, y: usize, _z: usize,
) -> usize {
    let dimensions = &field_conf.dimensions;
    let (dim_x, dim_y, _dim_z) = (dimensions[0], dimensions[1], dimensions[2]);

    let mut nr_of_neighbors = 0;
    for dx in [-1, 0, 1] {
        for dy in [-1, 0, 1] {
           // prevent self-interaction
           if (dx == 0) && (dy == 0) { continue; }
           // get coordinates of other cell
           let X = apply_periodic_bounds(x as i32 + dx, dim_x as i32);
           let Y = apply_periodic_bounds(y as i32 + dy, dim_y as i32);
           let cell_idx = (Y * dim_x as i32 + X) as usize;
           // increment number of neighbors
           if field.entries[cell_idx] == 1. { nr_of_neighbors += 1; }
        }
    }
    nr_of_neighbors
}


pub fn step_cell(
    field: &mut Field,
    config: &EngineConfig,
    states: &Vec<State>,
    x: usize, 
    y: usize, 
    z: usize,
) {
    // get about field info from conf
    let field_conf = &config.fields[field.id];
    let cell_idx = get_cell_idx_from_coords(x, y, z, &field, &field_conf);
    let last_field = &states[config.iter_idx].fields[field.id];
    // numerical parameters TODO
    let T = 0.01;
    // math setup
    let mut rng = rand::thread_rng();

    // FIELD-FIELD INTERACTIONS
    let field_interactions = &field_conf.field_interactions;
    for interaction in field_interactions.iter() {
        match interaction {
            FieldFieldInteraction::Ising => {
                let dE = get_flip_energy(
                    config.iter_idx, field, &field_conf, &states, x, y, z
                );
                // check if spin should be flipped
                let flip = if dE < 0. { true } else {
                    let rand: f64 = rng.gen();
                    if rand < boltzmann_prob(dE, T) { true } else { false }
                };
                // flip spin
                if flip { field.entries[cell_idx] *= -1.; }  // TODO generalize to 3D

            }, FieldFieldInteraction::GameOfLife => {

                let nr_of_neighbors = get_nr_of_neighbors(&last_field, &field_conf, x, y, 0);
                let next = match nr_of_neighbors {
                    2 => if field.entries[cell_idx] == 1. { 1. } else { 0. }, 
                    3 => 1., 
                    _ => 0.
                };
                field.entries[cell_idx] = next;

            }, _ => {}
        }
    }
    // FIELD-OBJ INTERACTIONS
    let _obj_interactions = &config.fields[field.id].obj_interactions;

    // ...
}


pub fn step(
    field: &mut Field,
    states: &Vec<State>,
    config: &EngineConfig,
) {
    // math setup
    let mut rng = rand::thread_rng();
    // get info about field from config
    let field_conf = &config.fields[field.id];
    let dimensions = &field_conf.dimensions;
    let (dim_x, dim_y, dim_z) = (dimensions[0], dimensions[1], dimensions[2]); // TODO handle 3D
    let _field_interactions = &field_conf.field_interactions;
    let _obj_interactions = &field_conf.obj_interactions;

    match config.fields[field.id].relevant_cells {

        FieldRelevantCells::Entire => {
            for z in 0..dim_z {
                for y in 0..dim_y {
                    for x in 0..dim_x {
                        step_cell(field, config, states, x, y, z);
                    }
                }
            }

        }, FieldRelevantCells::RandomBatch => {
            const BATCH_SIZE: usize = 1000; // TODO where to get batch-size from?
            for _ in 0..BATCH_SIZE {
                let x = rng.gen_range(0..dim_x-1);
                let y = rng.gen_range(0..dim_y-1);
                let z = 0; // TODO rng.gen_range(0..dim_z);
                step_cell(field, config, states, x, y, z);
            }
        }
    }
}

