
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::simulation::Simulation;

use mxyz_utils::dom;


pub fn get_slider_value(slider_id: &str) -> String {
    dom::document()
        .get_element_by_id(slider_id)
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
}

#[wasm_bindgen]
impl Simulation {

    pub fn handle_slider_event(&mut self, slider_id: &str) {
        dom::console::log(&format!("slider-id: {}", slider_id));

        match slider_id {
            "slider_set-iterations-per-render" => {
                // get value from slider
                let val = get_slider_value(slider_id)
                    .parse::<usize>()
                    .unwrap();
                // scale logarithmically from 1 to 10**max_pow
                let max_pow = 2.;
                let steps = 10_f64.powf(val as f64 / 100. * max_pow);  // TODO make sim_id-dependent
                self.config.nr_of_steps_per_render = steps as usize;
            }, _ => {}
        };
    }

}
