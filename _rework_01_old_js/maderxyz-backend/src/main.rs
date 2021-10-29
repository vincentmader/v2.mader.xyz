// SETUP
// ============================================================================

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
// use rocket::fs::FileServer;

use serde::{Serialize, Deserialize};

extern crate tera;

// VIEWS
// ============================================================================

// index
// ----------------------------------------------------------------------------

// struct NavGridSection {
//     title: String, 
//     // subsections: Vec,
// }
// impl NavGridSection {
//     fn new(title: String) -> NavGridSection {
//         NavGridSection {
//             title
//         }
//     }
// }


#[derive(Serialize, Deserialize, Clone)]
struct Section {
    id: String,
    title: String,
    path_to_thumbnail: String,
    // subsections
}
impl Section {
    pub fn new(
        id: &str, 
        title: &str,
    ) -> Section {
        let mut path_to_thumbnail = String::from("/static/media/thumbnails/");
        path_to_thumbnail.push_str(&id);
        path_to_thumbnail.push_str(".png");
        return Section {
            id: String::from(id),
            title: String::from(title),
            path_to_thumbnail: path_to_thumbnail, 
        };
    }
}

#[derive(Serialize, Deserialize)]
struct FancyPants<'a> {
    #[serde(borrow)]
    sections: Vec<&'a str>,
    #[serde(borrow)]
    subsections: HashMap<&'a str, Vec<Section>>,
    // sections: &'a Vec<Section>,
    // sections: HashMap<&'a str, &'a Section>,
}

#[get("/sims/<id>")]
fn sims(id: String) -> Template {
    let title = String::from(&id);  // TODO: get title
    let context: HashMap<&str, &String> = [
        ("id", &id),
        ("title", &title),
    ].iter().cloned().collect();
    Template::render(format!("sims/{}", id), &context)
}

#[get("/")]
fn index() -> Template {

    let subsections: HashMap<&str, Vec<Section>> = [
        ("emergent behavior & cellular automata", [
            Section::new("ants", "ants"),
            // Section::new("ant war"),
            Section::new("boids", "boids"),
            Section::new("game_of_life", "game of life"),
            // Section::new("", "light thingies"),
        ].iter().cloned().collect()), 
        ("gravitational n-body dynamics", [
            // Section::new("", "2-body interaction"),
            Section::new("3body_moon", "3-body interaction: moon"),
            Section::new("3body_fig8", "3-body interaction: figure-8"),
            // Section::new("nbody_cloud", "3-body interaction: stellar cluster"),
        ].iter().cloned().collect()), 
        ("oscillators", [
            Section::new("single_pendulum", "single pendulum"),
            Section::new("double_pendulum", "double pendulum"),
            Section::new("lissajous", "Lissajous figures"),
        ].iter().cloned().collect()), 
        // ("fluid dynamics", [
            // Section::new("diffusion", "diffusion"),
            // Section::new("incompressible fluid"),
            // Section::new("flow through pipe"),
        // ].iter().cloned().collect()), 
        ("electro-magnetism", [
            Section::new("charge_interaction", "charged-particle interaction"),
            Section::new("lorentz", "Wien filter"),
        ].iter().cloned().collect()), 
        ("statistical physics & thermodynamics", [
            Section::new("thermal_motion", "thermal motion"),
            Section::new("brownian_motion", "Brownian motion"),
            Section::new("ising", "Ising model"),
        ].iter().cloned().collect()), 
        ("various", [
            Section::new("forest_fire", "forest fire"),
            Section::new("rock_paper_scissors", "rock-paper-scissors"),
            Section::new("nbody_asteroids", "n-body interaction: asteroids around binary"),
            Section::new("quad_tree", "quad-tree"),
            Section::new("tatooine", "Tatooine"),
            Section::new("mc_pi_darts", "Monte Carlo pi calculation"),
            Section::new("draw", "canvas drawing demo"),
            Section::new("react", "react.js frontend"),
            Section::new("diffusion", "diffusion"),
            Section::new("incompressible_fluid", "incompressible fluid"),
            Section::new("game_of_life_wasm", "game of life"),
            Section::new("wasm", "wasm"),
        ].iter().cloned().collect()), 
    ].iter().cloned().collect();

    // let mut sections: Vec<&str> = Vec::new();
    // for (k, _) in &subsections {
    //     sections.push(k);
    // }
    // sections.sort();
    let sections = vec![
        "gravitational n-body dynamics",
        "emergent behavior & cellular automata",
        "oscillators",
        "statistical physics & thermodynamics",
        "electro-magnetism",
        "various",
    ];

    let fp: FancyPants = FancyPants {
        sections,
        subsections,
        // sections: &vec![
        //     // (
        //     // String::from("oscillators"), 
        //     Section {
        //         title: String::from("double pendulum")
        //     }
        //     // ),
        // ].iter().cloned().collect(),
    };

    let context: HashMap<&str, &FancyPants> = [
        ("fp", &fp)
    ].iter().cloned().collect();
    Template::render("index", &context)
}

// MAIN
// ============================================================================

fn main() {
    rocket::ignite()
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![index, sims])
        .attach(Template::fairing())
        .launch();
}



