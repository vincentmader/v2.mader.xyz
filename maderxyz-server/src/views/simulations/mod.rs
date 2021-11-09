// pub mod nbody_gravity;
use std::collections::HashMap;

extern crate rocket_dyn_templates;
use rocket_dyn_templates::Template;


#[get("/simulations/<page_id>")]
pub fn routes(page_id: &str) -> Template {

    let template = match page_id {
        "3body_fig8" => "simulations/nbody",
        "ising" | "diffusion" => "simulations/cellular_automaton",
        _ => "simulations/base",
    };

    let context: HashMap<&str, &str> = [
        ("page_id", page_id),
        ("title", page_id),
    ].iter().cloned().collect();

    Template::render(template, &context)
}
