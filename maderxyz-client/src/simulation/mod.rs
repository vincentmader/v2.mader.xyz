
use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod renderer;
use renderer::Renderer;
use renderer::ObjectColorMode;
use crate::utils;
use maderxyz_numerics::engine::Engine;


#[wasm_bindgen]
pub struct Simulation {
    engine: Engine,
    renderer: Renderer,
    // parameters: Parameters,
    parameters: HashMap<String, f64>,
}
#[wasm_bindgen]
impl Simulation {
    pub fn new(page_id: &str) -> Self {
        // utils::dom::set_panic_hook(); // TODO: helpful?
        let engine = Engine::new(page_id);
        let renderer = Renderer::new(page_id);
        // let parameters = Parameters::new();
        let parameters = HashMap::new();
        Simulation { engine, renderer, parameters }

    }
    pub fn init(&mut self) {

        self.engine.init();
        self.renderer.init();
        self.init_buttons();
        self.init_sliders();

    }
    pub fn step(&mut self) {  // TODO: multithread & async

        self.renderer.display(&self.engine.states);
        for _ in 0..1{  // TODO
            self.engine.step();
        }

    }
    pub fn init_buttons(&mut self) {

        let button_ids = Vec::from([
            "reset",
            // "toggle-pause",
            "toggle-pause-engine",
            "toggle-pause-renderer",
            "set-obj_color_mode-hsv_velocity",
            "set-obj_color_mode-hsv_position",
            "set-obj_color_mode-speed",
            "set-obj_color_mode-distance_from_origin",
            // "toggle-display-tails",
            // "set-tail_type-default",
            // "set-tail_type-area",
        ]);
        for button_id in button_ids {
            let document = utils::dom::document();
            let section = document.get_element_by_id("button_menu-0").unwrap();
	        let button = document.create_element("button").unwrap()
	    	    .dyn_into::<web_sys::HtmlButtonElement>().unwrap();
            button.set_id(&format!("button-{}", button_id));
	        section.append_child(&button).unwrap();
        }

    }
    pub fn init_sliders(&mut self) {

        let slider_ids = Vec::from([
            "tail-length",
        ]);
        for slider_id in slider_ids {
            let document = utils::dom::document();
            let section = document.get_element_by_id("button_menu-0").unwrap();
	        let slider = document.create_element("input").unwrap()
	    	    .dyn_into::<web_sys::HtmlInputElement>().unwrap();
            slider.set_id(&format!("slider-{}", slider_id));
            slider.set_attribute("type", "range").unwrap();
            slider.set_attribute("min", "0").unwrap();
            slider.set_attribute("max", "1000").unwrap();
            slider.set_attribute("value", "500").unwrap();
	        section.append_child(&slider).unwrap();
        }

    }
    pub fn handle_button_event(&mut self, button_id: &str) {

        let document = utils::dom::document();
        let button = document.get_element_by_id(button_id).unwrap();

        match button_id {
            // "button-toggle-pause" => {
            //     self.renderer.pause();
            //     self.engine.toggle_pause();
            // },
            "button-reset" => {
                self.engine.init();
                self.engine.iteration_step = 0;
                self.renderer.frame_idx = 0;
            },
            "button-toggle-pause-engine" => {
                self.engine.is_paused = !self.engine.is_paused;
                match self.engine.is_paused {
                    true => button.set_text_content(Some("unpause")), 
                    false => button.set_text_content(Some("pause"))
                }
            }, "button-toggle-pause-renderer" => {
                self.renderer.is_paused = !self.renderer.is_paused;
                match self.renderer.is_paused {
                    true => button.set_text_content(Some("unpause")),
                    false => button.set_text_content(Some("pause"))
                }
            }, "button-set-obj_color_mode-hsv_position" => {
                self.renderer.object_color_mode = ObjectColorMode::HSLPosition
            }, "button-set-obj_color_mode-hsv_velocity" => {
                self.renderer.object_color_mode = ObjectColorMode::HSLVelocity
            }, "button-set-obj_color_mode-speed" => {
                self.renderer.object_color_mode = ObjectColorMode::Speed
            }, "button-set-obj_color_mode-distance_from_origin" => {
                self.renderer.object_color_mode = ObjectColorMode::Distance
            }
            _ => {}
        };
        utils::dom::console_log(&format!("{}", button_id));
    }
    pub fn handle_slider_event(&mut self, slider_id: &str) {

        match slider_id {
            _ => {}
        };
        utils::dom::console_log(&format!("{}", slider_id));
    }
}


// struct Parameters {
//     G_newton: f64,
//     k_coulomb: f64,
// }


