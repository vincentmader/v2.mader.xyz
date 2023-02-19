
use std::collections::HashMap;

use physical_constants;


pub fn get_fundamental_constants() -> HashMap<&'static str, f64> {
    HashMap::from([
        ("c",       physical_constants::SPEED_OF_LIGHT_IN_VACUUM),
        ("k_B",     physical_constants::BOLTZMANN_CONSTANT),
        ("h_bar",   physical_constants::PLANCK_CONSTANT),
        ("G",       physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION),
    ])
}

