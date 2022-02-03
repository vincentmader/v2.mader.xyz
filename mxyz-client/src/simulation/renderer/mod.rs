
pub mod config;
pub mod field;
pub mod object;

mod buttons;



use std::collections::HashMap;
use std::cmp;

use mxyz_engine::Engine;
use mxyz_engine::state::State;
use mxyz_engine::state::field::Field;
use mxyz_engine::state::field::FieldVariant;
use mxyz_engine::state::object::ObjectFamily;

pub use mxyz_utils::dom::canvas::Canvas;
pub use mxyz_utils::dom::console;

use object::tail_variant::ObjectTailVariant;
use object::color_mode::ObjectColorMode;


pub struct Renderer {

    sim_id: String,
    pub frame_idx: usize,
    canvases: Vec<Canvas>,

    // pub is_paused: bool,
    // pub is_halted: bool,
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
            // is_paused: false,
            // is_halted: false,
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

    pub fn init(&mut self, engine: &Engine) {
        let states = &engine.states;

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

    pub fn display(&mut self, engine: &Engine) {
        let states = &engine.states;

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
            self.display_hud( &engine )  // todo
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
        canvas_id: usize,
    ) {

        let canvas = &mut self.canvases[canvas_id];

        // let grid_size

        for entry in field.entries.iter() {

            match field.variant {
                FieldVariant::Ising => {

                }, _ => {}
            }

            // match self.sim_id {
            // }

        }

        // let canvas = &mut self.canvases[canvas_id];
        // console::log("aaa");

        // let (x, y, r) = (0., 0., 0.1);
        // canvas.draw_circle((x,y), r, true);

    }

    pub fn display_hud(
        &mut self, engine: &Engine,
    ) {
        let canvas = &mut self.canvases[0];
        let ctx = &canvas.context;
        canvas.set_font("36px sans-serif");

        let W = canvas.dimensions.0;
        let H = canvas.dimensions.1;

        canvas.set_stroke_style("white");
        canvas.set_fill_style("white");

        let frame_idx = format!("{}", self.frame_idx);
        canvas.fill_text(&frame_idx, 20., 50.);

        // let iteration_idx = format!("{}", engine.iteration_idx); 
        // canvas.fill_text(&iteration_idx, 20., 100.);

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

