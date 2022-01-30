
pub mod config;
pub mod field;
pub mod object;


use std::collections::HashMap;
use std::cmp;

pub use mxyz_engine::state::State;
pub use mxyz_engine::state::field::Field;
pub use mxyz_engine::state::object::ObjectFamily;

pub use mxyz_utils::dom::canvas::Canvas;
pub use mxyz_utils::dom::console;

use object::tail_variant::ObjectTailVariant;
use object::color_mode::ObjectColorMode;


pub struct Renderer {

    sim_id: String,
    pub frame_idx: usize,
    canvases: Vec<Canvas>,

    pub is_paused: bool,
    pub is_halted: bool,
    pub is_displaying_hud: bool,
    pub is_clearing_canvas: bool,

    pub is_drawing_families: Vec<bool>,
    pub is_drawing_pos_vec: Vec<bool>,
    pub is_drawing_vel_vec: Vec<bool>,
    pub is_drawing_acc_vec: Vec<bool>,
    pub obj_tail_variant: Vec<ObjectTailVariant>,
    pub obj_color_mode: Vec<ObjectColorMode>,

}
impl Renderer {

    pub fn new(sim_id: &str) -> Self {
        Renderer {
            sim_id: String::from(sim_id),
            frame_idx: 0,
            canvases: Vec::new(),
            //
            is_paused: false,
            is_halted: false,
            is_displaying_hud: false,
            is_clearing_canvas: true,
            is_drawing_families: Vec::new(),
            is_drawing_pos_vec: Vec::new(),
            is_drawing_vel_vec: Vec::new(),
            is_drawing_acc_vec: Vec::new(),
            obj_tail_variant: Vec::new(),
            obj_color_mode: Vec::new(),
            // TODO tail pre-computes (?)
        }
    }

    pub fn init(&mut self, states: &Vec<State>) {
        let doc = mxyz_utils::dom::document();

        // TODO generalize canvas initialization
        let canvas = Canvas::new("canvas_main");
        self.canvases.push(canvas);

        let initial_state = &states[self.frame_idx];
        let nr_of_families = initial_state.object_families.len();

        // setup default object-family rendering options
        for idx in 0..nr_of_families {
            self.is_drawing_families.push(true);
            self.is_drawing_pos_vec.push(false);
            self.is_drawing_vel_vec.push(false);
            self.is_drawing_acc_vec.push(false);

            let obj_tail_variant = match self.sim_id.as_str() {
                // "2body-kepler" => ObjectColorMode::Speed,
                // "nbody-flowers" => ObjectColorMode::Distance,
                "nbody-asteroids" => ObjectTailVariant::None,
                _ => ObjectTailVariant::Line,
            };
            let obj_color_mode = match self.sim_id.as_str() {
                "nbody-misc" => ObjectColorMode::Speed,
                "nbody-asteroids" => ObjectColorMode::Distance,
                // "nbody-flowers" => ObjectColorMode::Distance,
                _ => ObjectColorMode::Default,
            };
            self.obj_tail_variant.push(obj_tail_variant);
            self.obj_color_mode.push(obj_color_mode);
        }
        // TODO setup default field rendering options

        // TODO initialize user inputs
        self.init_buttons(&initial_state);
        self.init_sliders(&initial_state);

        self.init_button_menu_1(&doc, );
        self.init_button_menu_2(&doc, &initial_state);
        
    }

    fn init_button_menu_1(
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

    fn init_button_menu_2(
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
            ])), ("object interactions", Vec::from([
                ("button_set-obj-interaction-none", "none"),
                ("button_set-obj-interaction-newton", "Newton"),
                ("button_set-obj-interaction-coulomb", "Coulomb"),
                ("button_set-obj-interaction-lennard-jones", "L.J."),
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

    pub fn display(&mut self, states: &Vec<State>) {
        // TODO use engine here instead of engine.states

        // STATE SETUP
        let current_state = &states[self.frame_idx];
        let fields = &current_state.fields;
        let families = &current_state.object_families;

        // CANVAS SETUP
        let canvas_id = 0;  // todo: get id 
        let canvas = &mut self.canvases[canvas_id];
        if self.is_clearing_canvas { canvas.clear(); }

        // DISPLAY FIELDS
        for field in fields.iter() {
            self.display_field( field, states, canvas_id );
        }

        // DISPLAY OBJECT FAMILIES
        for family in families.iter() {
            self.display_objects( family, states, canvas_id );
        }

        // DISPLAY HUD
        if self.is_displaying_hud { 
            self.display_hud()  // todo
        }
    }

    pub fn display_objects(
        &mut self, 
        family: &ObjectFamily,
        states: &Vec<State>,
        canvas_id: usize,
    ) {
        // const r: f64 = 0.01;  // TODO setup slider
        const r: f64 = 0.01;  // TODO setup slider
        const is_filled: bool = true;  // TODO setup toggle-button

        let objects = &family.objects;
        let object_length = family.object_length;
        let nr_of_objects = family.nr_of_objects;

        // SETUP CANVAS
        let canvas = &mut self.canvases[canvas_id];

        // SETUP OBJECT COLOR
        let object_color_mode = &self.obj_color_mode[family.id];
        let get_object_color = match object_color_mode {
            ObjectColorMode::Default     => object::color_mode::get_object_color_default,
            ObjectColorMode::Mass        => object::color_mode::get_object_color_from_mass,
            ObjectColorMode::HSLVelocity => object::color_mode::get_object_color_from_velocity_angle, 
            ObjectColorMode::HSLPosition => object::color_mode::get_object_color_from_position_angle, 
            ObjectColorMode::Speed       => object::color_mode::get_object_color_from_speed,
            ObjectColorMode::Distance    => object::color_mode::get_object_color_from_distance, // NOTE from origin
            ObjectColorMode::Charge      => object::color_mode::get_object_color_from_charge,
        };
        // loop over objects
        for obj_id in 0..nr_of_objects {
            let start_idx = obj_id * object_length;
            let obj = Vec::from(&objects[start_idx..start_idx+object_length]);
            // get color from color-mode
            let color = get_object_color(&obj, 1.);
            canvas.set_stroke_style(&color);
            canvas.set_fill_style(&color);

            // DISPLAY OBJECTs
            if self.is_drawing_families[family.id] {
                let (x, y) = (obj[1], obj[2]);
                canvas.draw_circle( (x, y), r, is_filled )
            }
            // DISPLAY POSITION VECTORs
            if self.is_drawing_pos_vec[family.id] {
                let (x, y) = (obj[1], obj[2]);
                canvas.draw_line((x, y), (0., 0.));
            }
            // DISPLAY VELOCITY VECTORs
            if self.is_drawing_vel_vec[family.id] {
                let (x, y, u, v) = (obj[1], obj[2], obj[3], obj[4]);
                let z = (u.powf(2.) + v.powf(2.)).powf(-0.5) / 5.; // TODO make configurable
                canvas.draw_line((x, y), (x+u*z, y+v*z));
            }

            // DISPLAY ACCELERATION VECTOR
            if self.is_drawing_acc_vec[family.id] {
                // TODO
            }

        }

        // DISPLAY OBJECT TAILS
        let tail_variant = &self.obj_tail_variant[family.id];
        match tail_variant {
            ObjectTailVariant::Line => {     // LINE TAILS
                self.display_line_tails(&family, states, canvas_id);
            }, ObjectTailVariant::Area => {  // AREA TAILS
                self.display_area_tails(&family, states, canvas_id);
            }, _ => {
            }
        }

    }

    pub fn display_line_tails(
        &mut self,
        family: &ObjectFamily,
        states: &Vec<State>,
        canvas_id: usize,
    ) {
        const tail_length: usize = 100; // TODO make configurable

        let nr_of_objects = family.nr_of_objects;
        let object_length = family.object_length;

        // SETUP CANVAS
        let canvas = &mut self.canvases[canvas_id];

        // SETUP COLOR
        let object_color_mode = &self.obj_color_mode[family.id];
        let get_object_color = match object_color_mode {
            ObjectColorMode::Default     => object::color_mode::get_object_color_default,
            ObjectColorMode::Mass        => object::color_mode::get_object_color_from_mass,
            ObjectColorMode::HSLVelocity => object::color_mode::get_object_color_from_velocity_angle, 
            ObjectColorMode::HSLPosition => object::color_mode::get_object_color_from_position_angle, 
            ObjectColorMode::Speed       => object::color_mode::get_object_color_from_speed,
            ObjectColorMode::Distance    => object::color_mode::get_object_color_from_distance, // NOTE from origin
            ObjectColorMode::Charge      => object::color_mode::get_object_color_from_charge,
        };

        let iterator = 0..usize::min(tail_length, self.frame_idx);
        for tail_step_id in iterator.rev() {

            for obj_id in 0..nr_of_objects {
                let start_idx = obj_id*object_length;

                let state = &states[self.frame_idx - tail_step_id];
                let obj = &state.object_families[family.id].objects[start_idx..start_idx+object_length];
                let previous_state = &states[self.frame_idx - tail_step_id - 1];
                let previous_obj = &previous_state.object_families[family.id].objects[start_idx..start_idx+object_length];

                let (x1, y1) = (previous_obj[1], previous_obj[2]);
                let (x2, y2) = (obj[1], obj[2]);

                // setup color
                let alpha = 1. - tail_step_id as f64 / tail_length as f64;
                let color = get_object_color( &Vec::from(obj), alpha );
                canvas.set_stroke_style(&color);
                canvas.set_fill_style(&color);

                canvas.draw_line( (x1, y1), (x2, y2) );

            }  // TODO apply alpha   (from input?)
        }
    }

    pub fn display_area_tails(
        &mut self,
        family: &ObjectFamily,
        states: &Vec<State>,
        canvas_id: usize,
    ) {
        let canvas = &mut self.canvases[canvas_id];
        let tail_length = 200; // TODO make configurable

        // setup color
        let object_color_mode = &self.obj_color_mode[family.id];
        let get_object_color = match object_color_mode {
            ObjectColorMode::Default     => object::color_mode::get_object_color_default,

            ObjectColorMode::Mass        => object::color_mode::get_object_color_from_mass,
            ObjectColorMode::HSLVelocity => object::color_mode::get_object_color_from_velocity_angle, 
            ObjectColorMode::HSLPosition => object::color_mode::get_object_color_from_position_angle, 
            ObjectColorMode::Speed       => object::color_mode::get_object_color_from_speed,
            ObjectColorMode::Distance    => object::color_mode::get_object_color_from_distance, // NOTE from origin
            ObjectColorMode::Charge      => object::color_mode::get_object_color_from_charge,
        };

        let nr_of_objects = family.nr_of_objects;
        let object_length = family.object_length;

        let iterator = 0..usize::min(tail_length, self.frame_idx);
        for tail_step_id in iterator.rev() {

            for obj_id in 0..nr_of_objects {
                let start_idx = obj_id*object_length;

                let state = &states[self.frame_idx - tail_step_id];
                let obj = &state.object_families[family.id].objects[start_idx..start_idx+object_length];
                let previous_state = &states[self.frame_idx - tail_step_id - 1];
                let previous_obj = &previous_state.object_families[family.id].objects[start_idx..start_idx+object_length];

                let (x1, y1) = (previous_obj[1], previous_obj[2]);
                let (x2, y2) = (obj[1], obj[2]);
                let (x3, y3) = (0., 0.);

                // setup color
                let alpha = 1. - tail_step_id as f64 / tail_length as f64;
                let color = get_object_color( &Vec::from(obj), alpha );
                canvas.set_stroke_style(&color);
                canvas.set_fill_style(&color);

                canvas.draw_triangle( (x1, y1), (x2, y2), (x3, y3) )

            }  // TODO apply alpha   (from input?)
        }
    }

    pub fn display_field(
        &mut self, 
        field: &Field, 
        states: &Vec<State>,
        canvas_id: usize
    ) {

        // let canvas = &mut self.canvases[canvas_id];
        // console::log("aaa");

        // let (x, y, r) = (0., 0., 0.1);
        // canvas.draw_circle((x,y), r, true);

    }

    pub fn display_hud(
        &mut self,
    ) {

    }

    pub fn reset(&mut self) {
        self.frame_idx = 0;  // TODO this does not reset engine (?)
        for canvas in self.canvases.iter_mut() {
            canvas.clear();
        }
    }

    pub fn create_button_menu_for_object_family(
        &mut self, 
        object_family: &ObjectFamily,
    ) {

    }

    pub fn create_button_menu_for_field(
        &mut self, 
        field: &Field
    ) {

    }

    // todo: rename -> init_button_menus
    pub fn init_buttons(&mut self, state: &State) {

        // ================

        

        // ================

        struct Input {

        }

        enum InputVariant {
            button,
            option,
            slider,
        }

        // ================


        // ================


        // ================

        // let slider_ids = Vec::from([
        //     "slider_dt"
        // ]);
    }

    pub fn init_sliders(&mut self, state: &State) {

    }




}

