
use rand::{Rng};

use crate::interactions::field::Interaction as FieldInteraction;


pub fn interact(

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

