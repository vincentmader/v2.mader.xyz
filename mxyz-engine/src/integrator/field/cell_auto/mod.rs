#![allow(non_snake_case)]

use rand::Rng;

use crate::state::State;
use crate::state::field::Field;

// use crate::interaction::field as interactions;
use crate::config::EngineConfig;
use crate::config::field::FieldEngineConfig;
use crate::state::field::variant::FieldVariant;

use mxyz_physics::thermo_dynamics::boltzmann_prob;


pub fn apply_periodic_bounds(
    field_conf: &FieldEngineConfig, 
    X: i32, 
    Y: i32
) -> (i32, i32) {

    let mut X = X;
    let mut Y = Y;

    let dimensions = &field_conf.dimensions;
    let (dim_x, dim_y, _dim_z) = (dimensions[0], dimensions[1], dimensions[2]); // TODO handle 3D
    // apply periodic bounds
    if X < 0 { X += dim_x as i32; }
    if Y < 0 { Y += dim_y as i32; }
    if X >= dim_x as i32 { X -= dim_x as i32; }
    if Y >= dim_y as i32 { Y -= dim_y as i32; }
    (X, Y)
}

pub fn get_flip_energy(
    iter_idx: usize,
    field: &mut Field,
    field_conf: &FieldEngineConfig, 
    states: &Vec<State>,
    x: usize, 
    y: usize
) -> f64 {
    let (B, J, mu) = (0., 1., 1.);

    let dimensions = &field_conf.dimensions;
    let dim_x = dimensions[0]; // TODO handle 3D

    let cell = field.entries[y*dim_x+x];
    let mut dE = 0.;
    for dx in 0..3 {
        for dy in 0..3 {
            // prevent self-interaction
            if (dx == 1) && (dy == 1) { continue; }
            // get coordinates of other cell
            let (X, Y) = apply_periodic_bounds(&field_conf, x as i32 + dx-1, y as i32 + dy-1);

            // get other cell 
            let other = states[iter_idx].fields[field.id].entries[Y as usize*dim_x+X as usize];
            // add spin-spin interaction to flip energy
            dE += J*cell*other;
        }
    }
    // add spin-field interaction to flip energy
    dE + mu*cell*B
}


pub fn step(
    field: &mut Field,
    states: &Vec<State>,
    config: &EngineConfig,
) {
    // numerical parameters
    const BATCH_SIZE: usize = 1000; // TODO where to get batch-size from?
    
    let iter_idx = config.iter_idx;

    // math setup
    let mut rng = rand::thread_rng();
    // get info about field from config
    let field_conf = &config.fields[field.id];
    let _field_interactions = &field_conf.field_interactions;
    let _obj_interactions = &field_conf.obj_interactions;
    let dimensions = &field_conf.dimensions;
    let (dim_x, dim_y, _dim_z) = (dimensions[0], dimensions[1], dimensions[2]); // TODO handle 3D

    let T = 0.01;

    for _ in 0..BATCH_SIZE {
        // choose random cell
        let x = rng.gen_range(0..dim_x);
        let y = rng.gen_range(0..dim_y);
        // loop over neighbors

        match field_conf.field_variant {
            FieldVariant::Ising => {
                let dE = get_flip_energy(iter_idx, field, &field_conf, &states, x, y);
                mxyz_utils::dom::console::log(&format!("{}", dE));

                // flip spin
                let mut flip = false;
                if dE < 0. { flip = true; } else {
                    let rand: f64 = rng.gen();
                    if rand < boltzmann_prob(dE, T) {
                        flip = true;
                    }
                }
                if flip {
                    field.entries[y as usize*dim_x+x as usize] *= -1.;
                }
            }, FieldVariant::GameOfLife => {
                // field.entries[y as usize*dim_x+x as usize] *= -1.;

                // let N = get_neighbor_count()

            }
        }
    }
}

