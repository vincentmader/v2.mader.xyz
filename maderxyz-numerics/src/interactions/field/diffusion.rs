
use crate::interactions::field::Interaction as FieldInteraction;


pub fn interact(

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

