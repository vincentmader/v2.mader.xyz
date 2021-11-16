use std::collections::HashMap;

extern crate rocket_dyn_templates;
use rocket_dyn_templates::Template;

// use serde::{Serialize, Deserialize};


#[get("/simulations/3body_moon")]
pub fn moon() -> Template {

    let context: HashMap<&str, &str> = [
        ("title", "hello moon"),
    ].iter().cloned().collect();

    Template::render(
        "simulations/nbody_gravity/moon",
        // "index/index",
        &context
    )
}
