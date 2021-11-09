use std::collections::HashMap;

extern crate rocket_dyn_templates;
use rocket_dyn_templates::Template;

use serde::{Serialize, Deserialize};

mod section;
use section::Section;


// #[get("/")]
// pub fn index() -> Template {

//     let context: HashMap<&str, &str> = [
//         ("msg", "hello, static!")
//     ].iter().cloned().collect();

//     Template::render("index/index", &context)
// }


#[derive(Serialize, Deserialize)]
struct FancyPants<'a> {
    #[serde(borrow)]
    sections: Vec<&'a str>,
    #[serde(borrow)]
    subsections: HashMap<&'a str, Vec<Section>>,
    // sections: &'a Vec<Section>,
    // sections: HashMap<&'a str, &'a Section>,
}




#[get("/")]
pub fn route() -> Template {

    let subsections: HashMap<&str, Vec<Section>> = [
        ("emergent behavior", [
            // Section::new("ant war"),
            Section::new("boids", "boids"),
            Section::new("ants", "ants"),
            Section::new("game_of_life", "game of life"),
            // Section::new("", "light thingies"),
        ].iter().cloned().collect()), 
        ("gravitational dynamics", [
            // Section::new("", "2-body interaction"),
            Section::new("3body_moon", "3-body interaction: moon"),
            Section::new("3body_fig8", "3-body interaction: figure-8"),
            Section::new("nbody_flowers", "symmetric satellite constellation"),
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
        ("thermodynamics", [
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
        "gravitational dynamics",
        "oscillators",
        "thermodynamics",
        "emergent behavior",
        "electro-magnetism",
        "various",
    ];

    let fp: FancyPants = FancyPants {
        sections,
        subsections,
    };

    let context: HashMap<&str, &FancyPants> = [
        ("fp", &fp),
    ].iter().cloned().collect();
    Template::render("index/index", &context)
}
