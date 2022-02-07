
use rand::Rng;

use crate::state::State;
use crate::state::field::Field;

use crate::interaction::field as interactions;


fn boltzmann_prob(dE: f64, T: f64) -> f64 {  // TODO -> physics
    let kB = 1.;
    let E_th = kB * T;
    (-dE / E_th).exp()
}


pub fn step(
    iteration_idx: usize,
    field: &mut Field,
    states: &Vec<State>,
) {
    const BATCH_SIZE: usize = 1000;
    // TODO where to get batch-size from?

    let mut rng = rand::thread_rng();

    let dimensions = &field.dimensions;
    let dim_x = dimensions.0;
    let dim_y = dimensions.0;
    // let dim_z = dimensions.0;

    for _ in 0..BATCH_SIZE {
        let x = rng.gen_range(0..dim_x);
        let y = rng.gen_range(0..dim_y);

        let cell = field.entries[y*dim_x+x];

        let B = 0.;
        let T = 0.01;
        let J = 1.;
        let mu = 1.;

        let mut dE = 0.;
        for dx in 0..3 {
            for dy in 0..3 {
                if (dx == 1) && (dy == 1) { continue; }
                let mut X = x as i32 + dx - 1;
                let mut Y = y as i32 + dy - 1;
                
                // apply periodic bounds
                if X < 0 { X += dim_x as i32; }
                if Y < 0 { Y += dim_y as i32; }
                if X >= dim_x as i32 { X -= dim_x as i32; }
                if Y >= dim_y as i32 { Y -= dim_y as i32; }

                let other = states[iteration_idx].fields[field.id].entries[Y as usize*dim_x+X as usize];
                dE += J*cell*other;
            }
        }
        dE += mu*cell*B;

        let rand: f64 = rng.gen();
        if dE < 0.  {
            field.entries[y as usize*dim_x+x as usize] *= -1.;
        } else if rand < boltzmann_prob(dE, T) {
            field.entries[y as usize*dim_x+x as usize] *= -1.;
        }
    }
}

