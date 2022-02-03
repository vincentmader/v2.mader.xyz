
use mxyz_engine::state::State;

use crate::simulation::renderer::Renderer;


impl Renderer {

    pub fn init_button_menu_1(
        &mut self, 
        doc: &web_sys::Document
    ) {

        let button_ids = Vec::from([
            ("button_toggle-pause", "| |"),
            ("button_toggle-pause-engine", "|e|"),
            ("button_toggle-pause-renderer", "|r|"),
            ("button_toggle-halt-renderer", "h_r"),
            ("button_reset", "reset"),
            ("button_toggle-display-hud", "hud"),
            ("button_toggle-clear-canvas", "clear canvas"),
        ]);

        let button_menus = doc.get_element_by_id("button-menu_main").unwrap();
        let button_menu = doc.create_element("div").unwrap();
        button_menu.set_id(&format!("bm-{}", 0));  // default
        // button_menu.set_attribute("class", "section"); // not needed, defined in tera
        button_menus.append_child(&button_menu).unwrap();
        for entry in button_ids {
            let button_id = entry.0;
            let title = entry.1;
            let button = doc.create_element("button").unwrap();
            button.set_id(button_id);
            button.set_attribute("class", "bm_button").unwrap();
            button.set_inner_html(title);
            button_menu.append_child(&button).unwrap();
            // let button = document.get_element_by_id(button_id).unwrap();
            // button.set_inner_html("ayyy");
        }
    }

    pub fn init_button_menu_2(
        &mut self, 
        doc: &web_sys::Document, 
        state: &State
    ) {

        let buttons = Vec::from([
            ("button_toggle-display-tail", "display tails"),
        ]);
        let multibuttons = Vec::from([
            ("object motion vectors", Vec::from([
                ("button_toggle-display-obj-vec-pos", "pos"),
                ("button_toggle-display-obj-vec-vel", "vel"),
                ("button_toggle-display-obj-vec-acc", "acc"),
            // ])), ("object interactions", Vec::from([
            //     ("button_set-obj-interaction-none", "none"),
            //     ("button_set-obj-interaction-newton", "Newton"),
            //     ("button_set-obj-interaction-coulomb", "Coulomb"),
            //     ("button_set-obj-interaction-lennard-jones", "L.J."),
                // ("button_set-obj-interaction-boid", "boid"),
                // ("button_set-obj-interaction-boid", "collision"),
            ])),
        ]);
        let options = Vec::from([
            ("variant", Vec::from([
                ("button_set-obj-variant-static", "static"),
                ("button_set-obj-variant-body", "body"),
                ("button_set-obj-variant-particle", "particle"),
            ])), ("color mode", Vec::from([
                ("button_set-obj-col-default", "default"),
                ("button_set-obj-col-hsv-vel", "hsv vel"),
                ("button_set-obj-col-hsv-pos", "hsv pos"),
                ("button_set-obj-col-speed", "speed"),
                ("button_set-obj-col-dist", "distance"),
                ("button_set-obj-col-charge", "charge"),
                ("button_set-obj-col-mass", "mass"),
            ])), ("tail variant", Vec::from([
                ("button_set-obj-tail-variant-none", "none"),
                ("button_set-obj-tail-variant-line", "line"),
                ("button_set-obj-tail-variant-area", "area"),
            ])), ("integrator", Vec::from([
                ("button_set-obj-integrator-euler-imp", "euler imp."),
                ("button_set-obj-integrator-euler-exp", "euler exp."),
                ("button_set-obj-integrator-rk4", "rk4"),
                ("button_set-obj-integrator-rk2", "rk2"),
                ("button_set-obj-integrator-verlet", "verlet"),
                ("button_set-obj-integrator-leapfrog", "leapfrog"),
            ])), ("boundary conditions", Vec::from([
                ("button_set-obj-bound-periodic", "periodic"),
                ("button_set-obj-bound-wall-elastic", "elastic (w)"),
                ("button_set-obj-bound-wall-inelastic", "inelastic (w)"),
            ])),
        ]);

        struct Button {
            id: String,
            class: String,
        }

        struct BMOption {
            id: String,
            class: String,
        }

        struct BMMultibutton {
            id: String,
            class: String,
            buttons: Vec<Button>
        }

        let button_menus = doc.get_element_by_id("page-column-right").unwrap();
        for object_family in &state.object_families {

            // create section for object family
            let section = doc.create_element("div").unwrap();
            section.set_attribute("class", "section").unwrap();
            button_menus.append_child(&section).unwrap();
            // add title
            let title = doc.create_element("div").unwrap();
            title.set_inner_html(&format!("object family {}", &object_family.id));
            title.set_attribute("style", "text-align: left; padding-left: 10px;").unwrap();
            section.append_child(&title).unwrap();
            // create button menu in section, holding options & multi-buttons
            let button_menu = doc.create_element("div").unwrap();
            button_menu.set_attribute("class", "bm").unwrap();
            section.append_child(&button_menu).unwrap();

            // loop over multibuttons
            for (title, buttons) in &multibuttons {

                let multibutton_wrapper = doc.create_element("div").unwrap();
                button_menu.append_child(&multibutton_wrapper).unwrap();

                let title2 = doc.create_element("span").unwrap();
                title2.set_attribute("class", "bm_multibutton_title").unwrap();
                title2.set_inner_html(&format!("{}", &title));
                multibutton_wrapper.append_child(&title2).unwrap();

                let nr_of_buttons = buttons.len();
                let multibutton = doc.create_element("span").unwrap();
                multibutton.set_attribute("class", "bm_multibutton").unwrap();
                multibutton.set_attribute("style", &format!(
                    "grid-template-columns: repeat({}, 1fr);", nr_of_buttons
                )).unwrap();
                multibutton_wrapper.append_child(&multibutton).unwrap();

                // loop over multibutton buttons
                for button_idx in 0..nr_of_buttons {
                    let button = doc.create_element("button").unwrap();
                    button.set_attribute("class", "bm_multibutton_button").unwrap();
                    let button_id = buttons[button_idx].0;
                    let button_title = buttons[button_idx].1;
                    button.set_id(&format!("obj-fam_{}_{}", object_family.id, button_id));
                    button.set_inner_html(button_title);
                    if button_idx == 0 {
                        button.set_attribute("style", "border-left: 0px;").unwrap();
                    }
                    multibutton.append_child(&button).unwrap();
                }
            }

            let button_menu = doc.create_element("div").unwrap();
            button_menu.set_attribute("class", "bm_options").unwrap();
            section.append_child(&button_menu).unwrap();

            // let button_menu_wrapper = document.create_element("div").unwrap();
            // button_menu_wrapper.append_child(&button_menu).unwrap();
            // loop over options
            for (title, buttons) in &options {

                let div_option = doc.create_element("div").unwrap();
                div_option.set_attribute("class", "bm_option").unwrap();
                
                let div_option_title = doc.create_element("span").unwrap();
                div_option_title.set_attribute("class", "bm_option_title").unwrap();
                div_option_title.set_inner_html(&format!("{}", &title));
                div_option.append_child(&div_option_title).unwrap();

                let div_option_dropdown_container = doc.create_element("span").unwrap();
                div_option_dropdown_container.set_attribute("class", "dropdown-container").unwrap();
                div_option.append_child(&div_option_dropdown_container).unwrap();

                let div_option_dropdown = doc.create_element("div").unwrap();
                div_option_dropdown.set_attribute("class", "dropdown").unwrap();
                div_option_dropdown_container.append_child(&div_option_dropdown).unwrap();

                let div_option_dropdown_value = doc.create_element("div").unwrap();
                div_option_dropdown_value.set_inner_html("test");
                // div_option_dropdown_value.set_attribute("class", "dropdown").unwrap();
                div_option_dropdown.append_child(&div_option_dropdown_value).unwrap();

                // let option = document.create_element("span").unwrap();
                // option.set_attribute("class", "bm_option").unwrap();
                // option.set_attribute("style", &format!(
                //     "grid-template-columns: repeat({}, 1fr);", nr_of_buttons
                // )).unwrap();
                // button_menu.append_child(&option).unwrap();

                let div_option_dropdown_content = doc.create_element("span").unwrap();
                div_option_dropdown_content.set_attribute("class", "dropdown-content").unwrap();
                div_option_dropdown.append_child(&div_option_dropdown_content).unwrap();

                // loop over option buttons
                let nr_of_buttons = buttons.len();
                for button_idx in 0..nr_of_buttons {
                    let button_id = buttons[button_idx].0;
                    let button_title = buttons[button_idx].1;

                    let div_option_dropdown_item = doc.create_element("button").unwrap();
                    div_option_dropdown_item.set_attribute("class", "dropdown-item").unwrap();
                    div_option_dropdown_item.set_id(&format!("obj-fam_{}_{}", object_family.id, button_id));
                    div_option_dropdown_item.set_inner_html(button_title);
                //     // if button_idx == 0 {
                //     //     div_button_dropdown_item.set_attribute("style", "border-left: 0px;").unwrap();
                //     // }
                    div_option_dropdown_content.append_child(&div_option_dropdown_item).unwrap();
                }

                button_menu.append_child(&div_option).unwrap();
                section.append_child(&button_menu).unwrap();
            }

            // for foo in &buttons {
            //     let button = document.create_element("div").unwrap();
            //     button.set_attribute("class", "button");
            //     button.set_id(foo.0);
            //     button.set_inner_html(foo.1);
            //     button_menu.append_child(&button);
            // } 
        }

        let object_families = &state.object_families;
        for object_family in object_families {
            self.create_button_menu_for_object_family(object_family);
        }
        let fields = &state.fields;
        for field in fields {
            self.create_button_menu_for_field(field);
        }
    }

}
