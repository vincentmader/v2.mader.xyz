
pub mod index;

// use crate::utils::load_config;

// ----------

use std::collections::HashMap;

extern crate rocket_dyn_templates;
use rocket_dyn_templates::Template;


#[get("/simulations/<category>/<page_id>")]
pub fn routes( category: &str, page_id: &str ) -> Template {

    // let config = load_config();
    // let pages = config.pages;
    // let context = &pages[page_id];

    let title = match page_id {
        "2body-kepler"       => "Kepler's laws",
        "3body-moon"         => "3body orbits - Moon",
        "3body-lagrange"     => "3body orbits - Lagrange points",
        "nbody-flowers"      => "symmetric satellite constellations",
        "nbody-binary"       => "binary star system",
        "nbody-asteroids"    => "asteroids around binary",
        "3body-fig8"         => "3body orbits - figure-8",
        "3body-broucke"      => "3body orbits - Broucke orbit",
        "charge-interaction" => "Coulomb interaction of charged particles",
        "ising-model"        => "Ising model",
        "diffusion"          => "fluid diffusion",
        "lennard-jones"      => "Lennard-Jones interaction",
        "game-of-life"       => "Conway's game of life",
                           _ => "TODO: title"
    };

    let context: HashMap<&str, &str> = [
        ("category", category),  
        ("page_id", page_id), 
        ("title", title), 
    ].iter().cloned().collect();

    let template = "simulations/base";
    Template::render(template, &context)
}

