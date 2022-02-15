#![allow(non_snake_case)]


pub fn boltzmann_prob(
    dE: f64, T: f64
) -> f64 { 

    let kB = 1.;
    let E_th = kB * T;
    (-dE / E_th).exp()

}

