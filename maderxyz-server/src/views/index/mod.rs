use std::collections::HashMap;

extern crate rocket_dyn_templates;
use rocket_dyn_templates::Template;

use serde::{Serialize, Deserialize};

// mod section;
// use section::Section;


// #[get("/")]
// pub fn index() -> Template {

//     let context: HashMap<&str, &str> = [
//         ("msg", "hello, static!")
//     ].iter().cloned().collect();

//     Template::render("index/index", &context)
// }


// #[derive(Serialize, Deserialize)]
// struct FancyPants<'a> {
//     #[serde(borrow)]
//     sections: Vec<&'a str>,
//     #[serde(borrow)]
//     subsections: HashMap<&'a str, Vec<Section>>,
//     // sections: &'a Vec<Section>,
//     // sections: HashMap<&'a str, &'a Section>,
// }






#[derive(Serialize, Deserialize)]
pub struct NavGrid {
    sections: Vec<NavGridSection>,
}
impl NavGrid {
    pub fn new() -> Self {

        let grid_section_titles: Vec<(&str, &str)> = Vec::from([
            ("gravity", "classical gravity"),
            // ("oscillators", "oscillators"),
            ("electro-magnetism", "electro-magnetism"),
            ("thermodynamics", "thermodynamics"),
            // ("emergent-behavior", "emergent behavior"),
            // ("various", "various"),
        ]);

        let mut sections: Vec<NavGridSection> = Vec::new();
        for (id, title) in grid_section_titles.iter() {
            sections.push(NavGridSection::new(id, title));
        }

        NavGrid {
            sections,
        } 
    }
}
#[derive(Serialize, Deserialize)]
pub struct NavGridSection {
    id: String,
    title: String,
    items: Vec<NavGridItem>,
}
impl NavGridSection {
    pub fn new(id: &str, title: &str) -> Self { 

        let items: Vec<NavGridItem> = match id {
            // "oscillators" => Vec::from([
            //     NavGridItem::new("single-pendulum"),
            //     NavGridItem::new("double-pendulum"),
            // ]),
            "gravity" => Vec::from([
                NavGridItem::new("3body-moon"),
                NavGridItem::new("nbody-flowers"),
                NavGridItem::new("nbody"),
                NavGridItem::new("3body-fig8"),
                NavGridItem::new("nbody-asteroids"),
                // NavGridItem::new("nbody"),
                // NavGridItem::new("nbody-binary"),
                // NavGridItem::new("nbody-cluster"),
            ]),
            "electro-magnetism" => Vec::from([
                NavGridItem::new("charge-interaction"),
            ]),
            "thermodynamics" => Vec::from([
                NavGridItem::new("ising"),
            ]),
            _ => Vec::new()
        };

        NavGridSection {
            id: String::from(id),
            title: String::from(title),
            items,
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct NavGridItem {
    id: String,
    path_to_thumbnail: String,
}
impl NavGridItem {
    pub fn new(id: &str) -> Self {

        let path_to_thumbnail = format!(
            "/static/img/thumbnails/{}.png", id
        );

        NavGridItem {
            id: String::from(id),
            path_to_thumbnail,
        }
    }
}


#[get("/")]
pub fn route() -> Template {

    // let subsections: HashMap<&str, Vec<Section>> = [
    //     ("emergent behavior", [
    //         // Section::new("ant war"),
    //         Section::new("boids", "boids"),
    //         Section::new("ants", "ants"),
    //         Section::new("game_of_life", "game of life"),
    //         // Section::new("", "light thingies"),
    //     ].iter().cloned().collect()), 
    //     ("gravitational dynamics", [
    //         // Section::new("", "2-body interaction"),
    //         Section::new("3body_moon", "3-body interaction: moon"),
    //         Section::new("nbody/3body_fig8", "3-body interaction: figure-8"),
    //         Section::new("nbody_flowers", "symmetric satellite constellation"),
    //         Section::new("nbody", "?"),
    //         Section::new("nbody_asteroids", "n-body interaction: asteroids around binary"),
    //         Section::new("nbody_cloud", "3-body interaction: stellar cluster"),
    //     ].iter().cloned().collect()), 
    //     ("oscillators", [
    //         Section::new("single_pendulum", "single pendulum"),
    //         Section::new("double_pendulum", "double pendulum"),
    //         Section::new("lissajous", "Lissajous figures"),
    //     ].iter().cloned().collect()), 
    //     // ("fluid dynamics", [
    //         // Section::new("diffusion", "diffusion"),
    //         // Section::new("incompressible fluid"),
    //         // Section::new("flow through pipe"),
    //     // ].iter().cloned().collect()), 
    //     ("electro-magnetism", [
    //         Section::new("charge_interaction", "charged-particle interaction"),
    //         Section::new("lorentz", "Wien filter"),
    //     ].iter().cloned().collect()), 
    //     ("thermodynamics", [
    //         Section::new("thermal_motion", "thermal motion"),
    //         Section::new("brownian_motion", "Brownian motion"),
    //         Section::new("ising", "Ising model"),
    //     ].iter().cloned().collect()), 
    //     ("various", [
    //         // Section::new("forest_fire", "forest fire"),
    //         Section::new("rock_paper_scissors", "rock-paper-scissors"),
    //         // Section::new("quad_tree", "quad-tree"),
    //         // Section::new("tatooine", "Tatooine"),
    //         Section::new("mc_pi_darts", "Monte Carlo pi calculation"),
    //         // Section::new("draw", "canvas drawing demo"),
    //         // Section::new("react", "react.js frontend"),
    //         Section::new("diffusion", "diffusion"),
    //         // Section::new("incompressible_fluid", "incompressible fluid"),
    //         // Section::new("game_of_life_wasm", "game of life"),
    //         // Section::new("wasm", "wasm"),
    //     ].iter().cloned().collect()), 
    // ].iter().cloned().collect();

    // let mut sections: Vec<&str> = Vec::new();
    // for (k, _) in &subsections {
    //     sections.push(k);
    // }
    // sections.sort();
    // let sections = vec![
    //     "oscillators",
    //     "gravitational dynamics",
    //     "electro-magnetism",
    //     "thermodynamics",
    //     "emergent behavior",
    //     "various",
    // ];

    let a = NavGrid::new();

    // let fp: FancyPants = FancyPants {
    //     sections,
    //     subsections,
    // };

    let context: HashMap<&str, &NavGrid> = [
        ("navgrid", &a),
    ].iter().cloned().collect();
    Template::render("index/index", &context)
}
