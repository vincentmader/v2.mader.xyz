#![allow(non_snake_case)]

use rand::Rng;

use crate::state::State;
use crate::state::field::Field;

use crate::interaction::field as interactions;
use crate::config::EngineConfig;

use mxyz_physics::thermo_dynamics::boltzmann_prob;


pub fn step(
    iter_idx: usize,
    field: &mut Field,
    states: &Vec<State>,
    config: &EngineConfig,
) {
    // math setup
    let mut rng = rand::thread_rng();
    // numerical parameters
    const BATCH_SIZE: usize = 1000; // TODO where to get batch-size from?
    let B = 0.;
    let T = 0.01;
    let J = 1.;
    let mu = 1.;
    // get field dimensions
    let dimensions = &config.fields[field.id].dimensions;
    let dim_x = dimensions[0];
    let dim_y = dimensions[1];
    let dim_z = dimensions[2];  // TODO handle

    for _ in 0..BATCH_SIZE {
        // choose random cell
        let x = rng.gen_range(0..dim_x);
        let y = rng.gen_range(0..dim_y);
        let cell = field.entries[y*dim_x+x];
        // calculate flip energy
        let mut dE = 0.;
        // loop over neighbors
        for dx in 0..3 {
            for dy in 0..3 {
                // prevent self-interaction
                if (dx == 1) && (dy == 1) { continue; }
                // get coordinates of other cell
                let mut X = x as i32 + dx - 1;
                let mut Y = y as i32 + dy - 1;
                // apply periodic bounds
                if X < 0 { X += dim_x as i32; }
                if Y < 0 { Y += dim_y as i32; }
                if X >= dim_x as i32 { X -= dim_x as i32; }
                if Y >= dim_y as i32 { Y -= dim_y as i32; }
                // get other cell 
                let other = states[iter_idx].fields[field.id].entries[Y as usize*dim_x+X as usize];
                // add spin-spin interaction to flip energy
                dE += J*cell*other;
            }
        }
        // add spin-field interaction to flip energy
        dE += mu*cell*B;

        let rand: f64 = rng.gen();
        if dE < 0.  {
            field.entries[y as usize*dim_x+x as usize] *= -1.;
        } else if rand < boltzmann_prob(dE, T) {
            field.entries[y as usize*dim_x+x as usize] *= -1.;
        }
    }
}

