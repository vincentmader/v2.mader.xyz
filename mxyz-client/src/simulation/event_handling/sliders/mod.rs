
use wasm_bindgen::prelude::*;

use crate::simulation::Simulation;


#[wasm_bindgen]
impl Simulation {

    pub fn handle_slider_event(&mut self, slider_id: &str) {

        match slider_id {
            _ => {}
        };
        mxyz_utils::dom::console::log(&format!("{}", slider_id));
    }

}
