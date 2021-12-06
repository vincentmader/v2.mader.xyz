
use crate::state::field;
use crate::state::field::Field;
use crate::state::field::NeighborhoodVariant;
use crate::interactions::field::Interaction;


pub fn step(

    field: &mut Field,
    interaction: &Interaction,

) {

    // setup neighborhood getter-function
    let neighborhood_getter = match field.neighborhood_variant {
        NeighborhoodVariant::Neumann => field::neighborhood_getter::get_neumann_neighborhood,
        NeighborhoodVariant::Moore => field::neighborhood_getter::get_moore_neighborhood
    };

    for row_idx in 0..field.dimensions.1 {
        for col_idx in 0..field.dimensions.0 {
            let position = (col_idx, row_idx);
            let neighborhood = neighborhood_getter(&field, position);
            let mut cell = &mut field.cells[row_idx*field.dimensions.1+col_idx];
            let mut neighbors: Vec<&Vec<f64>> = Vec::new();
            for neighbor_pos in neighborhood.iter() {
                let col_jdx = neighbor_pos.0;
                let row_jdx = neighbor_pos.1;
                let neighbor = &field.cells[row_jdx*field.dimensions.1+col_jdx];
                neighbors.push(neighbor);
            }
            // TODO apply interaction(s?)
        }
    }
}





