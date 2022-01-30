
use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use mxyz_engine::Engine;
mod event_handling;
mod config;
use config::Config;
mod renderer;
use renderer::Renderer;


#[wasm_bindgen]
pub struct Simulation {
    config: Config,
    engine: Engine,
    renderer: Renderer,
    is_paused: bool,
}

#[wasm_bindgen]
impl Simulation {

    pub fn new(sim_id: &str) -> Self {
        Simulation { 
            engine: Engine::new(&sim_id),
            renderer: Renderer::new(&sim_id),
            config: Config::new(&sim_id),
            is_paused: false,
        }
    }

    pub fn init(&mut self) {
        self.engine.init();
        self.renderer.init(&self.engine.states);  // TODO &self.engine instead
    }

    pub fn step(&mut self) {  // TODO: multi-thread & async
        match self.is_paused {
            true => {},
            false => { self.engine.step(); }
        }
    }

    pub fn render(&mut self) {
        let iteration_idx = self.engine.states.len();
        let out_of_bounds = self.renderer.frame_idx >= iteration_idx;
        let is_paused = self.config.is_paused || out_of_bounds;
        if !is_paused {
            self.renderer.frame_idx = iteration_idx - 1; // TODO increment += 1 ?
            self.renderer.display(&self.engine.states);
        }
    }
}

    // pub fn init_buttons(&mut self) {

    //     // general
    //     // object families
    //     // fields
        
    //     // fn init_option(
    //     //     doc: web_sys::Document,
    //     //     menu_id: &str,
    //     //     title: &str,
    //     //     button_ids: &HashMap<&str, &str>
    //     // ) {
    //     //     let menu = doc.get_element_by_id(menu_id).unwrap();

    //     //     let nr_of_buttons = button_ids.len();
    //     //     let multibutton = doc.create_element("select").unwrap();
    //     //     // multibutton.set_attribute("class", "multi-button").unwrap();
    //     //     // multibutton.set_attribute("style", &format!("grid-template-columns: repeat({}, 1fr)", nr_of_buttons)).unwrap();

    //     //     for (id, title) in button_ids.iter() {
	            // // let button = doc.create_element("option").unwrap()
	                // // .dyn_into::<web_sys::HtmlOptionElement>().unwrap();
    //     //         button.set_id(&format!("button-{}", id));
    //     //         button.set_inner_html(title);
    //     //         // button.set_attribute("title", title).unwrap();
    //     //         // button.set_attribute("class", "sub-button").unwrap();
	            // // multibutton.append_child(&button).unwrap();
    //     //     }
    //     //     menu.append_child(&multibutton).unwrap();
    //     // }

    //     fn init_multibutton(
    //         doc: web_sys::Document,
    //         menu_id: &str,
    //         title: &str,
    //         button_ids: &HashMap<&str, &str>
    //     ) {
    //         let menu = doc.get_element_by_id(menu_id).unwrap();

    //         let nr_of_buttons = button_ids.len();
    //         let multibutton = doc.create_element("div").unwrap();
    //         multibutton.set_attribute("class", "multi-button").unwrap();
    //         multibutton.set_attribute("style", &format!("grid-template-columns: repeat({}, 1fr)", nr_of_buttons)).unwrap();

    //         for (id, title) in button_ids.iter() {
	            // let button = doc.create_element("button").unwrap()
	                // .dyn_into::<web_sys::HtmlButtonElement>().unwrap();
    //             button.set_id(&format!("button-{}", id));
    //             button.set_inner_html(title);
    //             // button.set_attribute("title", title).unwrap();
    //             button.set_attribute("class", "sub-button").unwrap();
	            // multibutton.append_child(&button).unwrap();
    //         }
    //         menu.append_child(&multibutton).unwrap();
    //     }

    //     fn init_multibutton2(
    //         doc: web_sys::Document,
    //         menu_id: &str,
    //         title: &str,
    //         button_ids: &HashMap<&str, &str>
    //     ) {
    //         let menu = doc.get_element_by_id(menu_id).unwrap();

    //         // let nr_of_buttons = button_ids.len();
    //         let container = doc.create_element("div").unwrap();
    //         container.set_attribute("class", "dropdown-container").unwrap();
    //         let multibutton = doc.create_element("div").unwrap();
    //         multibutton.set_attribute("class", "dropdown").unwrap();
    //         multibutton.set_inner_html(title);
            
    //         let foo = doc.create_element("div").unwrap();
    //         foo.set_attribute("class", "dropdown-content").unwrap();

    //         for (id, title) in button_ids.iter() {
	            // let button = doc.create_element("button").unwrap()
	                // .dyn_into::<web_sys::HtmlButtonElement>().unwrap();
    //             button.set_id(&format!("button-{}", id));
    //             button.set_inner_html(title);
    //             // button.set_attribute("title", title).unwrap();
    //             button.set_attribute("class", "dropdown-item").unwrap();
	            // foo.append_child(&button).unwrap();
    //         }
    //         multibutton.append_child(&foo).unwrap();
    //         container.append_child(&multibutton).unwrap();
    //         menu.append_child(&container).unwrap();
    //     }

    //     let document = utils::dom::document();
    //     let menus = document.get_element_by_id("button-menus").unwrap();
    //     let menu = document.create_element("div").unwrap();
    //             menu.set_id(&format!("bm-{}", 0));
    //     menus.append_child(&menu).unwrap();
    //     let object_color_modes = HashMap::from([
    //         ("set-obj_color_mode-default", "def"),
    //         ("set-obj_color_mode-hsv_velocity", "hsv vel"),
    //         ("set-obj_color_mode-hsv_position", "hsv pos"),
    //         ("set-obj_color_mode-speed", "speed"),
    //         ("set-obj_color_mode-distance_from_origin", "pos"),
    //         ("set-obj_color_mode-charge", "charge"),
    //     ]);
    //     init_multibutton(
    //         document, "bm-0", "object color mode", &object_color_modes
    //     );
    //     let document = utils::dom::document();
    //     init_multibutton2(
    //         document, "bm-0", "object color mode", &object_color_modes
    //     );
    //     // let document = utils::dom::document();
    //     // init_option(
    //     //     document, "bm-0", "object color mode", &object_color_modes
    //     // );





    //     let buttons = HashMap::from([
    //         ("general", HashMap::from([
    //             ("reset", "reset"),
    //             // ("toggle-pause", "pause"),
    //             ("toggle-pause-engine", "pause engine"),
    //             ("toggle-pause-renderer", "pause renderer"),
    //         ])),
    //         ("tails", HashMap::from([
    //             ("toggle-display-tails", "display tails"),
    //             // ("set-tail_type-default", "def"),
    //             // ("set-tail_type-area", "area"),
    //         ])),
    //     ]);

    //     let document = utils::dom::document();
    //     let menus = document.get_element_by_id("button-menus").unwrap();
    //     for (menu_id, foo) in buttons {
    //         let menu = document.create_element("div").unwrap();
    //         menu.set_attribute("class", "button_menu").unwrap();
    //         let title = document.create_element("p").unwrap();
    //         title.set_inner_html(menu_id);
    //         menus.append_child(&title).unwrap();
    //         // let title = document.get_element_by_id(format!("p_{}", menu_id));
    //         for (button_id, button_title) in foo.iter() {
	            // let button = document.create_element("button").unwrap()
	        	    // .dyn_into::<web_sys::HtmlButtonElement>().unwrap();
    //             button.set_id(&format!("button-{}", button_id));
    //             button.set_inner_html(button_title);
    //             button.set_attribute("title", button_title).unwrap();
	            // menu.append_child(&button).unwrap();
    //         }
    //         menus.append_child(&menu).unwrap();
    //     }
    // }
    // pub fn init_sliders(&mut self) {

    //     let slider_ids: Vec<&str> = Vec::from([
    //         // "tail-length",
    //     ]);
    //     for slider_id in slider_ids {
    //         let document = utils::dom::document();
    //         let section = document.get_element_by_id("button_menu-0").unwrap();
	        // let slider = document.create_element("input").unwrap()
	    	    // .dyn_into::<web_sys::HtmlInputElement>().unwrap();
    //         slider.set_id(&format!("slider-{}", slider_id));
    //         slider.set_attribute("type", "range").unwrap();
    //         slider.set_attribute("min", "0").unwrap();
    //         slider.set_attribute("max", "1000").unwrap();
    //         slider.set_attribute("value", "500").unwrap();
	        // section.append_child(&slider).unwrap();
    //     }

    // }
    // // pub fn handle_option_event(&mut self, option_id: &str) {

    // //     let document = utils::dom::document();
    // //     let option = document.get_element_by_id(option_id).unwrap();

    // //     match option_id {
    // //         "button-set-obj_color_mode-default" => {
    // //             // button.set_attribute("class", "sub-button_active").unwrap();
    // //             self.renderer.object_color_mode = ObjectColorMode::Default;
    // //         }, "button-set-obj_color_mode-hsv_position" => {
    // //             self.renderer.object_color_mode = ObjectColorMode::HSLPosition;
    // //         }, "button-set-obj_color_mode-hsv_velocity" => {
    // //             self.renderer.object_color_mode = ObjectColorMode::HSLVelocity;
    // //         }, "button-set-obj_color_mode-speed" => {
    // //             self.renderer.object_color_mode = ObjectColorMode::Speed;
    // //         }, "button-set-obj_color_mode-distance_from_origin" => {
    // //             self.renderer.object_color_mode = ObjectColorMode::Distance;
    // //         }, "button-set-obj_color_mode-charge" => {
    // //             self.renderer.object_color_mode = ObjectColorMode::Charge;
    // //         }, _ => {}
    // //     }
    // }
