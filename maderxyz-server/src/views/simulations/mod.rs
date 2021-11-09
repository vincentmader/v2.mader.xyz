// pub mod nbody_gravity;
use std::collections::HashMap;

extern crate rocket_dyn_templates;
use rocket_dyn_templates::Template;


#[get("/simulations/<sim_id>")]
pub fn routes(sim_id: &str) -> Template {

    let context: HashMap<&str, &str> = [
        ("sim_id", sim_id),
        ("title", sim_id),
    ].iter().cloned().collect();

    let template = match sim_id {
        "3body_fig8" => "simulations/nbody",
        "ising" => "simulations/cellular_automaton",
        _ => "simulations/base",
    };

    Template::render(template, &context)
}
