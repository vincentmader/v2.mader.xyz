pub mod index;

// ----------

use std::collections::HashMap;

extern crate rocket_dyn_templates;
use rocket_dyn_templates::Template;


#[get("/simulations/<category>/<page_id>")]
pub fn routes( category: &str, page_id: &str ) -> Template {

    let title = match page_id {
        "3body-fig8" => "3-body orbits: figure-8",
        "3body-moon" => "3-body orbits: Moon",
        "ising" => "Ising model",
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
