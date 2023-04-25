
use wasm_bindgen::prelude::*;

use mxyz_engine::boundary::object::variant::ObjBoundaryVariant;
use mxyz_engine::integrator::object::variant::ObjIntegratorVariant;
use mxyz_engine::state::object::variant::ObjVariant;
use mxyz_utils::dom::console;
use crate::simulation::Simulation;
use crate::simulation::renderer::object::color_mode::ObjColorMode;
use crate::simulation::renderer::object::tail_variant::ObjTailVariant;


#[wasm_bindgen]
impl Simulation {

    pub fn handle_button_event(&mut self, button_id: &str) {
        console::log(&format!("button-id: {}", button_id));

        let mut rel_button_id = String::from(button_id);
        // get id of field/obj_family that button belongs to
        let mut thing_id: usize = 0;
        if button_id.starts_with("obj-fam_") || button_id.starts_with("field_") {
            // id
            thing_id = button_id.split("_").nth(1).unwrap().parse::<usize>().unwrap();
            // get button_id without obj_family id    TODO make this prettier
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
                engine.reset();
                renderer.reset();
            }, 
            "button_toggle-pause" => {
                engine.config.is_paused = !engine.config.is_paused;
                renderer.config.is_paused = !renderer.config.is_paused;
            },
            "button_toggle-pause-engine" => {
                engine.config.is_paused = !engine.config.is_paused;
            },
            "button_toggle-pause-renderer" => {
                renderer.config.is_paused = !renderer.config.is_paused;
            },
            "button_toggle-display-hud" => {
                renderer.config.is_displaying_hud = !renderer.config.is_displaying_hud;
            },
            "button_toggle-clear-canvas" => {
                renderer.config.is_clearing_canvas = !renderer.config.is_clearing_canvas;
            },
            "button_toggle-time-inversion" => {  // TODO
                // engine.config.dt *= -1.;
                renderer.config.is_iterating_forward = !renderer.config.is_iterating_forward;
            },

            // OBJECT VARIANT

            "button_set-obj-variant-particle" => {
                engine.config.obj_families[thing_id].obj_variant = ObjVariant::Particle;
            },
            "button_set-obj-variant-body" => {
                engine.config.obj_families[thing_id].obj_variant = ObjVariant::Body;
            },
            "button_set-obj-variant-static" => {
                engine.config.obj_families[thing_id].obj_variant = ObjVariant::Static;
            },

            // OBJECT COLOR MODE
            "button_set-obj-col-default" => {
                renderer.config.obj_families[thing_id].color_mode = ObjColorMode::Default;
            },
            "button_set-obj-col-dist" => {
                renderer.config.obj_families[thing_id].color_mode = ObjColorMode::Distance;
            },
            "button_set-obj-col-speed" => {
                renderer.config.obj_families[thing_id].color_mode = ObjColorMode::Speed;
            },
            "button_set-obj-col-mass" => {
                renderer.config.obj_families[thing_id].color_mode = ObjColorMode::Mass;
            },
            "button_set-obj-col-charge" => {
                renderer.config.obj_families[thing_id].color_mode = ObjColorMode::Charge;
            },
            "button_set-obj-col-hsv-pos" => {
                renderer.config.obj_families[thing_id].color_mode = ObjColorMode::HSLPosition;
            },
            "button_set-obj-col-hsv-vel" => {
                renderer.config.obj_families[thing_id].color_mode = ObjColorMode::HSLVelocity;
            },

            // OBJECT TAIL VARIANT

            "button_set-obj-tail-variant-none" => {
                renderer.config.obj_families[thing_id].tail_variant 
                    = ObjTailVariant::None;
            },
            "button_set-obj-tail-variant-line" => {
                renderer.config.obj_families[thing_id].tail_variant 
                    = ObjTailVariant::Line;
            },
            "button_set-obj-tail-variant-area" => {
                renderer.config.obj_families[thing_id].tail_variant 
                    = ObjTailVariant::Area;
            },

            // OBJECT MOTION VECTORS

            "button_toggle-display-objects" => {
                renderer.config.obj_families[thing_id].is_displaying_objects 
                    = !renderer.config.obj_families[thing_id].is_displaying_objects 
            },
            "button_toggle-display-obj-vec-pos" => {
                renderer.config.obj_families[thing_id].is_displaying_pos_vec 
                    = !renderer.config.obj_families[thing_id].is_displaying_pos_vec 
            },
            "button_toggle-display-obj-vec-vel" => {
                renderer.config.obj_families[thing_id].is_displaying_vel_vec 
                    = !renderer.config.obj_families[thing_id].is_displaying_vel_vec 
            },
            "button_toggle-display-obj-vec-acc" => {
                renderer.config.obj_families[thing_id].is_displaying_acc_vec 
                    = !renderer.config.obj_families[thing_id].is_displaying_acc_vec 
            },
            "button_toggle-display-obj-center-of-mass" => {
                renderer.config.obj_families[thing_id].is_displaying_center_of_mass 
                    = !renderer.config.obj_families[thing_id].is_displaying_center_of_mass 
            },
            // "button_toggle-display-obj-center-of-momentum" => {
            //     renderer.config.obj_families[thing_id].is_displaying_center_of_momentum 
            //         = !renderer.config.obj_families[thing_id].is_displaying_center_of_momentum 
            // },

            // FIELD VARIANT

            // ...

            // INTEGRATOR VARIANT    TODO
 
            "button_set-obj-integrator-euler-exp" => {
                engine.config.obj_families[thing_id].integrator 
                    = ObjIntegratorVariant::EulerExplicit;
            },
            "button_set-obj-integrator-euler-imp" => {},
            "button_set-obj-integrator-rk2" => {},
            "button_set-obj-integrator-rk4" => {},
            "button_set-obj-integrator-leapfrog" => {},
            "button_set-obj-integrator-verlet" => {},

            // BOUNDARY VARIANT

            "button_set-obj-bound-none" => {  // TODO
                engine.config.obj_families[thing_id].boundary 
                    = ObjBoundaryVariant::None;
            },
            "button_set-obj-bound-periodic" => {
                engine.config.obj_families[thing_id].boundary 
                    = ObjBoundaryVariant::Periodic;
            },
            "button_set-obj-bound-wall-elastic" => {
                engine.config.obj_families[thing_id].boundary 
                    = ObjBoundaryVariant::WallCollisionElastic;
            },
            "button_set-obj-bound-wall-inelastic" => {
                engine.config.obj_families[thing_id].boundary 
                    = ObjBoundaryVariant::WallCollisionInelastic;
            },

            // ...

            "button_set-obj-interaction-none" => {
                // let nr_of_families = self.engine.states[self.engine.iter_idx].obj_families.len();
                // for idx in 0..nr_of_families {
                //     // engine.engine_setup.object[0].obj_interactions;
                // }
                // TODO save interactions not in enum, but struct
                    // e.g.     grav: true, coulomb: true, lj: false, ...
                    //     -> nope!  (?) },
            }, _ => { console::log("ERROR: button-handler not found"); }
        };
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
    //     let obj_color_modes = HashMap::from([
    //         ("set-obj_color_mode-default", "def"),
    //         ("set-obj_color_mode-hsv_velocity", "hsv vel"),
    //         ("set-obj_color_mode-hsv_position", "hsv pos"),
    //         ("set-obj_color_mode-speed", "speed"),
    //         ("set-obj_color_mode-distance_from_origin", "pos"),
    //         ("set-obj_color_mode-charge", "charge"),
    //     ]);
    //     init_multibutton(
    //         document, "bm-0", "object color mode", &obj_color_modes
    //     );
    //     let document = utils::dom::document();
    //     init_multibutton2(
    //         document, "bm-0", "object color mode", &obj_color_modes
    //     );
    //     // let document = utils::dom::document();
    //     // init_option(
    //     //     document, "bm-0", "object color mode", &obj_color_modes
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
    // //             self.renderer.obj_color_mode = ObjColorMode::Default;
    // //         }, "button-set-obj_color_mode-hsv_position" => {
    // //             self.renderer.obj_color_mode = ObjColorMode::HSLPosition;
    // //         }, "button-set-obj_color_mode-hsv_velocity" => {
    // //             self.renderer.obj_color_mode = ObjColorMode::HSLVelocity;
    // //         }, "button-set-obj_color_mode-speed" => {
    // //             self.renderer.obj_color_mode = ObjColorMode::Speed;
    // //         }, "button-set-obj_color_mode-distance_from_origin" => {
    // //             self.renderer.obj_color_mode = ObjColorMode::Distance;
    // //         }, "button-set-obj_color_mode-charge" => {
    // //             self.renderer.obj_color_mode = ObjColorMode::Charge;
    // //         }, _ => {}
    // //     }
    // }
