pub mod index;

use crate::utils::load_config;

// ----------

use std::collections::HashMap;

extern crate rocket_dyn_templates;
use rocket_dyn_templates::Template;


#[get("/simulations/<category>/<page_id>")]
pub fn routes( category: &str, page_id: &str ) -> Template {

    let title = match page_id {
        "3body-fig8" => "3-body orbits: figure-8",
        "3body-moon" => "3-body orbits: Moon",
        "nbody-flowers" => "symmetrical satellite constellation",
        "nbody-asteroids" => "asteroids around binary",
        "nbody-binary" => "binary star system",
        "3body-broucke" => "Broucke orbit",
        "charge-interaction" => "interaction of charged particles",
        "ising-model" => "Ising model",
        "diffusion" => "fluid diffusion",
        _ => "TODO: title"
    };

    // let config = load_config();
    // let pages = config.pages;
    // let context = &pages[page_id];

    let context: HashMap<&str, &str> = [
        ("category", category),  
        ("page_id", page_id), 
        ("title", title),  
    ].iter().cloned().collect();

    let template = "simulations/base";
    Template::render(template, &context)
}
