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
    section_id: String,
    title: String,
    items: Vec<NavGridItem>,
}
impl NavGridSection {
    pub fn new(section_id: &str, title: &str) -> Self { 

        let items: Vec<NavGridItem> = match section_id {
            // "oscillators" => Vec::from([
            //     NavGridItem::new("single-pendulum"),
            //     NavGridItem::new("double-pendulum"),
            // ]),
            "gravity" => Vec::from([
                NavGridItem::new("3body-moon", "Moon"),
                NavGridItem::new("nbody-flowers", "sym. constellations"),
                NavGridItem::new("nbody-solar", "solar system"),
                NavGridItem::new("3body-fig8", "figure-8"),
                NavGridItem::new("nbody-asteroids", "asteroids"),
                NavGridItem::new("nbody-binary", "binary"),
                NavGridItem::new("3body-broucke", "broucke"),
                // NavGridItem::new("3body-liao", "liao"),
                // NavGridItem::new("3body-freefall", "free-fall"),
                // NavGridItem::new("3body-moth", "moth"),
            ]),
            "electro-magnetism" => Vec::from([
                NavGridItem::new("charge-interaction", "charge interaction"),
            ]),
            "thermodynamics" => Vec::from([
                NavGridItem::new("ising-model", "Ising model"),
            ]),
            _ => Vec::new()
        };

        NavGridSection {
            section_id: String::from(section_id),
            title: String::from(section_id), // TODO -> title
            items,
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct NavGridItem {
    item_id: String,
    title: String,
    path_to_thumbnail: String,
}
impl NavGridItem {
    pub fn new(item_id: &str, title: &str) -> Self {

        let path_to_thumbnail = format!(
            "/static/img/thumbnails/{}.png", item_id
        );

        NavGridItem {
            item_id: String::from(item_id),
            title: String::from(title),
            path_to_thumbnail,
        }
    }
}


#[get("/")]
pub fn route() -> Template {
    let navgrid = NavGrid::new();
    let context: HashMap<&str, &NavGrid> = [
        ("navgrid", &navgrid),
    ].iter().cloned().collect();
    Template::render("index/index", &context)
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
}
