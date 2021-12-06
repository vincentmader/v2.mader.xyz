
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
    parameters: HashMap<String, String>,
}
#[wasm_bindgen]
impl Simulation {
    pub fn new(page_id: &str) -> Self {
        // utils::dom::set_panic_hook(); // TODO: helpful?

        let engine = Engine::new(page_id);
        let renderer = Renderer::new(page_id);
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
        // for _ in 0..10 {
            self.engine.step(&self.parameters);
        // }
    }
    pub fn render(&mut self) {
        self.renderer.display(&self.engine.states);
    }
    pub fn init_buttons(&mut self) {

        // general
        // object families
        // fields

        let buttons = HashMap::from([
            ("general", HashMap::from([
                ("reset", "reset"),
                // ("toggle-pause", "pause"),
                ("toggle-pause-engine", "pause engine"),
                ("toggle-pause-renderer", "pause renderer"),
            ])),
            ("object color mode", HashMap::from([
                // ("statics", ""), ... 
                // ("bodies", ""), ...
                // ("particles", ""), ...
                ("set-obj_color_mode-default", "def"),
                ("set-obj_color_mode-hsv_velocity", "hsv vel"),
                ("set-obj_color_mode-hsv_position", "hsv pos"),
                ("set-obj_color_mode-speed", "speed"),
                ("set-obj_color_mode-distance_from_origin", "pos"),
            ])),
            ("tails", HashMap::from([
                ("toggle-display-tails", "display tails"),
                // ("set-tail_type-default", "def"),
                // ("set-tail_type-area", "area"),
            ])),
        ]);

        // let button_ids = Vec::from([
        //     ("reset", "reset"),
        //     // ("toggle-pause", "pause"),
        //     ("toggle-pause-engine", "pause engine"),
        //     ("toggle-pause-renderer", "pause renderer"),
        //     ("set-obj_color_mode-default", "def"),
        //     ("set-obj_color_mode-hsv_velocity", "hsv vel"),
        //     ("set-obj_color_mode-hsv_position", "hsv pos"),
        //     ("set-obj_color_mode-speed", "speed"),
        //     ("set-obj_color_mode-distance_from_origin", "pos"),
        //     ("toggle-display-tails", "display tails"),
        //     // ("set-tail_type-default", "def"),
        //     // ("set-tail_type-area", "area"),
        // ]);
        let document = utils::dom::document();
        let menus = document.get_element_by_id("button-menus").unwrap();
        for (menu_id, foo) in buttons {
            let menu = document.create_element("div").unwrap();
            menu.set_attribute("class", "button_menu").unwrap();
            let title = document.create_element("p").unwrap();
            title.set_inner_html(menu_id);
            menus.append_child(&title).unwrap();
            // let title = document.get_element_by_id(format!("p_{}", menu_id));
            for (button_id, button_title) in foo.iter() {
	            let button = document.create_element("button").unwrap()
	        	    .dyn_into::<web_sys::HtmlButtonElement>().unwrap();
                button.set_id(&format!("button-{}", button_id));
                button.set_inner_html(button_title);
                button.set_attribute("title", button_title).unwrap();
	            menu.append_child(&button).unwrap();
            }
            menus.append_child(&menu).unwrap();
        }
    }
    pub fn init_sliders(&mut self) {

        let slider_ids: Vec<&str> = Vec::from([
            // "tail-length",
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
                    true => button.set_text_content(Some("unpause engine")), 
                    false => button.set_text_content(Some("pause engine"))
                }
            }, "button-toggle-pause-renderer" => {
                self.renderer.is_paused = !self.renderer.is_paused;
                match self.renderer.is_paused {
                    true => button.set_text_content(Some("unpause renderer")),
                    false => button.set_text_content(Some("pause renderer"))
                }
            }, "button-set-obj_color_mode-default" => {
                self.renderer.object_color_mode = ObjectColorMode::Default
            }, "button-set-obj_color_mode-hsv_position" => {
                self.renderer.object_color_mode = ObjectColorMode::HSLPosition
            }, "button-set-obj_color_mode-hsv_velocity" => {
                self.renderer.object_color_mode = ObjectColorMode::HSLVelocity
            }, "button-set-obj_color_mode-speed" => {
                self.renderer.object_color_mode = ObjectColorMode::Speed
            }, "button-set-obj_color_mode-distance_from_origin" => {
                self.renderer.object_color_mode = ObjectColorMode::Distance
            }, "button-toggle-display-tails" => {
                self.renderer.display_tails = !self.renderer.display_tails;
                match self.renderer.display_tails {
                    true => button.set_text_content(Some("hide tails")),
                    false => button.set_text_content(Some("display tails"))
                }
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


