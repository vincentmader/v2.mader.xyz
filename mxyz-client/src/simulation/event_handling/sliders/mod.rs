
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::simulation::Simulation;

use mxyz_utils::dom;
use mxyz_utils::dom::console;


#[wasm_bindgen]
impl Simulation {

    pub fn handle_slider_event(&mut self, slider_id: &str) {
        console::log(&format!("slider-id: {}", slider_id));

        let document = dom::document();

        match slider_id {
            "slider_set-iterations-per-render" => {
                self.config.nr_of_steps_per_render = document
                    .get_element_by_id(slider_id)
                    .unwrap()
                    .dyn_into::<HtmlInputElement>()
                    .unwrap()
                    .value()
                    .parse::<usize>()
                    .unwrap();
            }, _ => {}
        };
    }

}
