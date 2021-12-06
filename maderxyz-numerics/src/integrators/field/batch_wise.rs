
use rand::{Rng};

use crate::state::field;
use crate::state::field::Field;
use crate::state::field::NeighborhoodVariant;
use crate::interactions::field::Interaction;
use crate::interactions::field::InteractionVariant;


pub fn step(

    field: &mut Field,
    interaction: &Interaction,

) {

    let mut rng = rand::thread_rng();
    let batch_size = 5000; // TODO

    // setup neighborhood getter-function
    let neighborhood_getter = match field.neighborhood_variant {
        NeighborhoodVariant::Neumann => field::neighborhood_getter::get_neumann_neighborhood,
        NeighborhoodVariant::Moore => field::neighborhood_getter::get_moore_neighborhood
    };

    let interact = match interaction.interaction_variant {
        InteractionVariant::SpinSpin => {},
        InteractionVariant::Diffusion => {},
        InteractionVariant::GameOfLife => {},
    };

    for _ in 0..batch_size {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        let x = x * (*field).dimensions.0 as f64;
        let y = y * (*field).dimensions.1 as f64;
        let x = x as usize;
        let y = y as usize;

        let neighborhood = neighborhood_getter(&field, (x, y));
        let mut neighbors: Vec<&Vec<f64>> = Vec::new();
        let mut cell = &mut field.cells[y*field.dimensions.1+x];
        for neighbor_pos in neighborhood.iter() {
            let col_jdx = neighbor_pos.0;
            let row_jdx = neighbor_pos.1;
            let neighbor = &field.cells[row_jdx*field.dimensions.1+col_jdx];
            neighbors.push(neighbor);
        }
        // TODO apply interaction(s?)
    }
}
