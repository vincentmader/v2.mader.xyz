
use crate::state::field;
use crate::state::field::Field;
use crate::state::field::NeighborhoodVariant;
// use crate::interactions::field::Interaction;
use crate::interactions::field::InteractionVariant;
use crate::interactions::field::diffusion;
use crate::interactions::field::game_of_life;
use crate::interactions::field::spin_spin_interaction;


pub fn step(

    field: &mut Field,
    other_field: &Field,
    interactions: &Vec<InteractionVariant>,

) {

    // setup neighborhood getter-function
    let neighborhood_getter = match field.neighborhood_variant {
        NeighborhoodVariant::Neumann => field::neighborhood_getter::get_neumann_neighborhood,
        NeighborhoodVariant::Moore => field::neighborhood_getter::get_moore_neighborhood
    };

    for interaction in interactions.iter() {
        // setup interaction
        let interact = match interaction {
            InteractionVariant::SpinSpin => spin_spin_interaction::interact,
            InteractionVariant::Diffusion => diffusion::interact,
            InteractionVariant::GameOfLife => game_of_life::interact,
        };
        // loop over field entries
        for row_idx in 0..field.dimensions.1 { // TODO turn into one loop over cell_idx 0..N**2
            for col_idx in 0..field.dimensions.0 {
                let position = (col_idx, row_idx);
                // get neighborhood
                let neighborhood = neighborhood_getter(&field, position);
                let mut cell = &mut field.cells[row_idx*field.dimensions.1+col_idx];
                let mut neighbors: Vec<&Vec<f64>> = Vec::new();
                for neighbor_pos in neighborhood.iter() {
                    let col_jdx = neighbor_pos.0;
                    let row_jdx = neighbor_pos.1;
                    let neighbor = &other_field.cells[row_jdx*field.dimensions.1+col_jdx];
                    neighbors.push(neighbor);
                }
                interact(cell, &neighbors);
            }
        }
    } 
}





