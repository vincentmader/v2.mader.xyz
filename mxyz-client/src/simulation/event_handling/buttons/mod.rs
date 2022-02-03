
use wasm_bindgen::prelude::*;

use mxyz_engine::boundary::field::variant::BoundaryVariant as FieldBoundaryVariant;
use mxyz_engine::boundary::object::variant::BoundaryVariant as ObjectBoundaryVariant;
use mxyz_engine::integrator::field::variant::IntegratorVariant as FieldIntegratorVariant;
use mxyz_engine::integrator::object::variant::IntegratorVariant as ObjectIntegratorVariant;
use mxyz_engine::state::object::ObjectVariant;
use crate::simulation::Simulation;
use crate::simulation::renderer::object::color_mode::ObjectColorMode;
use crate::simulation::renderer::object::tail_variant::ObjectTailVariant;


#[wasm_bindgen]
impl Simulation {

    pub fn handle_button_event(&mut self, button_id: &str) {

        {
            use mxyz_utils::dom;
            dom::console::log(&format!("{}", button_id));
        }

        // let document = utils::dom::document();
        // let button = document.get_element_by_id(button_id).unwrap();

        let mut rel_button_id = String::from(button_id);
        // get id of field/object_family that button belongs to
        let mut thing_id: usize = 0;
        if button_id.starts_with("obj-fam_") || button_id.starts_with("field_") {
            // id
            let mut foo = button_id.split("_");
            thing_id = foo.nth(1).unwrap().parse::<usize>().unwrap();
            // get button_id without object_family id
            let mut foo = button_id.split("_");
            let count = button_id.split("_").count();
            rel_button_id = String::from("button");
            for idx in 3..count {
                rel_button_id = format!("{}_{}", rel_button_id, foo.nth(idx).unwrap());
            }
        }

        let engine = &mut self.engine;
        let renderer = &mut self.renderer;

        match rel_button_id.as_str() {

            // GENERAL (directly under canvas)

            "button_reset" => {
                engine.init();  // TODO instead: go back to first state, delete rest, use integrator setup
                engine.iteration_idx = 0;
                renderer.reset();
            }, 
            "button_toggle-pause" => {
                self.config.engine.is_paused = !self.config.engine.is_paused;
                self.config.renderer.is_paused = !self.config.renderer.is_paused;
            },
            "button_toggle-pause-engine" => {
                self.config.engine.is_paused = !self.config.engine.is_paused;
            },
            "button_toggle-pause-renderer" => {
                self.config.renderer.is_paused = !self.config.renderer.is_paused;
            },
            "button_toggle-display-hud" => {
                renderer.is_displaying_hud = !renderer.is_displaying_hud;
            },
            "button_toggle-clear-canvas" => {
                renderer.is_clearing_canvas = !renderer.is_clearing_canvas;
            },

            // OBJECT VARIANT

            "button_set-obj-variant-particle" => {
                let iteration_idx = engine.iteration_idx;
                engine.states[iteration_idx].object_families[thing_id].variant = ObjectVariant::Particle;
                // engine.config.obj_families[thing_id].obj_variant = ObjectVariant::Particle;
            },
            "button_set-obj-variant-body" => {
                let iteration_idx = engine.iteration_idx;
                engine.states[iteration_idx].object_families[thing_id].variant = ObjectVariant::Body;
            },
            "button_set-obj-variant-static" => {
                let iteration_idx = engine.iteration_idx;
                engine.states[iteration_idx].object_families[thing_id].variant = ObjectVariant::Static;
            },

            // OBJECT COLOR MODE
            "button_set-obj-col-default" => {
                renderer.obj_color_mode[thing_id] = ObjectColorMode::Default;
            },
            "button_set-obj-col-dist" => {
                renderer.obj_color_mode[thing_id] = ObjectColorMode::Distance;
            },
            "button_set-obj-col-speed" => {
                renderer.obj_color_mode[thing_id] = ObjectColorMode::Speed;
            },
            "button_set-obj-col-mass" => {
                renderer.obj_color_mode[thing_id] = ObjectColorMode::Mass;
            },
            "button_set-obj-col-charge" => {
                renderer.obj_color_mode[thing_id] = ObjectColorMode::Charge;
            },
            "button_set-obj-col-hsv-pos" => {
                renderer.obj_color_mode[thing_id] = ObjectColorMode::HSLPosition;
            },
            "button_set-obj-col-hsv-vel" => {
                renderer.obj_color_mode[thing_id] = ObjectColorMode::HSLVelocity;
            },

            // OBJECT TAIL VARIANT

            "button_set-obj-tail-variant-none" => {
                renderer.obj_tail_variant[thing_id] = ObjectTailVariant::None;
            },
            "button_set-obj-tail-variant-line" => {
                renderer.obj_tail_variant[thing_id] = ObjectTailVariant::Line;
            },
            "button_set-obj-tail-variant-area" => {
                renderer.obj_tail_variant[thing_id] = ObjectTailVariant::Area;
            },

            // OBJECT MOTION VECTORS

            "button_toggle-display-obj-vec-pos" => {
                renderer.is_drawing_pos_vec[thing_id] = !renderer.is_drawing_pos_vec[thing_id]
            },
            "button_toggle-display-obj-vec-vel" => {
                renderer.is_drawing_vel_vec[thing_id] = !renderer.is_drawing_vel_vec[thing_id]
            },
            "button_toggle-display-obj-vec-acc" => {
                renderer.is_drawing_acc_vec[thing_id] = !renderer.is_drawing_acc_vec[thing_id]
            },

            // FIELD VARIANT

            // ...

            // INTEGRATOR VARIANT

            "button_set-obj-integrator-euler-exp" => {
                engine.engine_setup.object_integrators[thing_id].variant = ObjectIntegratorVariant::EulerExplicit;
            },
            "button_set-obj-integrator-euler-imp" => {
                engine.engine_setup.object_integrators[thing_id].variant = ObjectIntegratorVariant::EulerImplicit;
            },
            "button_set-obj-integrator-rk2" => {
                engine.engine_setup.object_integrators[thing_id].variant = ObjectIntegratorVariant::RungeKutta2;
            },
            "button_set-obj-integrator-rk4" => {
                engine.engine_setup.object_integrators[thing_id].variant = ObjectIntegratorVariant::RungeKutta4;
            },
            "button_set-obj-integrator-leapfrog" => {
                engine.engine_setup.object_integrators[thing_id].variant = ObjectIntegratorVariant::LeapFrog;
            },
            "button_set-obj-integrator-verlet" => {
                engine.engine_setup.object_integrators[thing_id].variant = ObjectIntegratorVariant::Verlet;
            },

            // BOUNDARY VARIANT

            "button_set-obj-bound-none" => {  // TODO
                self.engine.engine_setup.object_boundaries[thing_id].variant = ObjectBoundaryVariant::None;
            },
            "button_set-obj-bound-periodic" => {
                engine.engine_setup.object_boundaries[thing_id].variant = ObjectBoundaryVariant::Periodic;
            },
            "button_set-obj-bound-wall-elastic" => {
                engine.engine_setup.object_boundaries[thing_id].variant = ObjectBoundaryVariant::WallCollisionElastic;
            },
            "button_set-obj-bound-wall-inelastic" => {
                engine.engine_setup.object_boundaries[thing_id].variant = ObjectBoundaryVariant::WallCollisionInelastic;
            },

            // ...

            "button_set-obj-interaction-none" => {
                // let nr_of_families = self.engine.states[self.engine.iteration_idx].object_families.len();
                // for idx in 0..nr_of_families {
                //     // self.engine.engine_setup.object[0].object_interactions;
                // }
                // TODO save interactions not in enum, but struct
                    // e.g.     grav: true, coulomb: true, lj: false, ...
                    //     -> nope!  (?) },
            }, _ => { mxyz_utils::dom::console::log("ERROR: button not found"); }
        };
        mxyz_utils::dom::console::log(&format!("{}", button_id));
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
