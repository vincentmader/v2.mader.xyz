
use crate::interactions::field::Interaction as FieldInteraction;


pub fn interact(

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

