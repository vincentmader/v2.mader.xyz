#![allow(non_snake_case)]


pub fn kepler_velocity(
    M: f64,
    r: f64
) -> f64 {
    let G = 1.;
    ( G*M/r ).sqrt()
}

