
pub mod config;
pub mod field;
pub mod object;
mod buttons;


use mxyz_engine::Engine;
use mxyz_engine::state::State;
use mxyz_engine::state::field::Field;
use mxyz_engine::state::object::ObjFamily;

pub use mxyz_utils::dom::canvas::Canvas;
pub use mxyz_utils::dom::console;

use object::tail_variant::ObjTailVariant;
use object::color_mode::ObjColorMode;


pub struct Renderer {

    sim_id:                     String,
    pub frame_idx:              usize,
    canvases:                   Vec<Canvas>,
    pub config:                 config::RendererConfig,

}

impl Renderer {

    pub fn new(sim_id: &str) -> Self {
        Renderer {
            sim_id:                 String::from(sim_id),
            frame_idx:              0,
            canvases:               Vec::new(),
            config:                 config::RendererConfig::new(sim_id),
            // TODO tail pre-computes (?)
        }
    }

    pub fn init(&mut self, engine: &Engine) {
        self.config.init(&engine);

        // TODO generalize canvas initialization
        let canvas = Canvas::new("canvas_main");
        self.canvases.push(canvas);

        // TODO initialize user inputs
        // self.init_buttons(&initial_state);
        // self.init_sliders(&initial_state);
        let doc = mxyz_utils::dom::document();
        self.init_button_menu_1(&doc, );
        let initial_state = &engine.states[self.frame_idx];
        self.init_button_menu_2(&doc, &initial_state);
        
    }

    pub fn display(&mut self, engine: &Engine) {
        let states = &engine.states;

        // STATE SETUP
        let current_state = &states[self.frame_idx];
        let fields = &current_state.fields;
        let families = &current_state.obj_families;

        // CANVAS SETUP
        let canvas_id = 0;  // todo: get id 
        let canvas = &mut self.canvases[canvas_id];
        if self.config.is_clearing_canvas { canvas.clear(); }

        // TODO put somewhere else
        let canvas = &mut self.canvases[canvas_id];
        // DISPLAY FIELD
        const FIELD_RESOLUTION: (usize, usize) = (40, 40);
        for row_idx in 0..FIELD_RESOLUTION.0 {
            for col_idx in 0..FIELD_RESOLUTION.1 {

                let x = (2.*(col_idx as f64 + 0.5) / FIELD_RESOLUTION.0 as f64) - 1.;  // TODO zoom
                let y = (2.*(row_idx as f64 + 0.5) / FIELD_RESOLUTION.1 as f64) - 1.;
                let (m, u, v, q) = (1., 0., 0., 1.);
                let mut force = Vec::from([0., 0.]);

                for family in families.iter() {
                    use mxyz_engine::state::object::variant::ObjVariant;
                    match engine.config.obj_families[family.id].obj_variant {
                        ObjVariant::Particle => { continue; },
                        _ => {}
                    }

                    let nr_of_objects = engine.config.obj_families[family.id].family_size;
                    let obj_length = &engine.config.obj_families[family.id].obj_attributes.len();
                    let objects = &family.objects;
                    for obj_id in 0..nr_of_objects {
                        let obj = Vec::from(&objects[obj_id*obj_length..(obj_id+1)*obj_length]);

                        use mxyz_engine::interaction::object::object::forces as obj_obj_forces;
                        let eps = 0.;

                        let force_getter = match self.sim_id.as_str() {
                            "lennard-jones" => obj_obj_forces::lennard_jones::force,
                            "charge-interaction" => obj_obj_forces::coulomb::force,
                            _ => obj_obj_forces::newtonian_gravity::force,
                        };

                        let f = force_getter(
                            &[m, x, y, u, v, q], &obj, eps,
                        );
                        force[0] += f[0];
                        force[1] += f[1];
                    }
                }

                let norm = (force[0].powf(2.) + force[1].powf(2.)).sqrt();
                let from = (x, y);
                let to = (x + force[0] / norm/FIELD_RESOLUTION.0 as f64, y + force[1] / norm/FIELD_RESOLUTION.1 as f64);

                let radius = 0.001;
                let max_force = 10.;  // TODO make slider
                let r = 255. * norm / max_force;
                let (g, b) = (r, r);
                let color = format!("rgb({}, {}, {})", r, g, b);
                canvas.set_stroke_style(&color);
                canvas.set_fill_style(&color);
                canvas.draw_line(from, to);
                canvas.draw_circle(to, radius, true);
            }
        }



        // DISPLAY FIELDS
        for field in fields.iter() {
            self.display_field(field, states, canvas_id, &engine);
        }
        // DISPLAY OBJECT FAMILIES
        for family in families.iter() {
            self.display_objects(family, states, canvas_id, &engine);
        }
        // DISPLAY HUD
        if self.config.is_displaying_hud { 
            self.display_hud(&engine);
        }

    }

    pub fn display_objects(
        &mut self, 
        family: &ObjFamily,
        states: &Vec<State>,
        canvas_id: usize,
        engine: &Engine,
    ) {

        // const r: f64 = 0.01;  // TODO setup slider
        // let r = 0.013;  // TODO setup slider
        let r = self.config.obj_families[family.id].obj_drawing_radius;
        let is_filled = true;  // TODO setup toggle-button

        let objects = &family.objects;
        let obj_length = engine.config.obj_families[family.id].obj_length;
        let nr_of_objects = engine.config.obj_families[family.id].family_size;

        // SETUP CANVAS
        let canvas = &mut self.canvases[canvas_id];

        // SETUP OBJECT COLOR
        let obj_color_mode = &self.config.obj_families[family.id].color_mode;
        let get_obj_color = match obj_color_mode {
            ObjColorMode::Default     => object::color_mode::get_obj_color_default,
            ObjColorMode::Mass        => object::color_mode::get_obj_color_from_mass,
            ObjColorMode::HSLVelocity => object::color_mode::get_obj_color_from_velocity_angle, 
            ObjColorMode::HSLPosition => object::color_mode::get_obj_color_from_position_angle, 
            ObjColorMode::Speed       => object::color_mode::get_obj_color_from_speed,
            ObjColorMode::Distance    => object::color_mode::get_obj_color_from_distance, // NOTE from origin
            ObjColorMode::Charge      => object::color_mode::get_obj_color_from_charge,
        };
        // loop over objects
        for obj_id in 0..nr_of_objects {
            let start_idx = obj_id * obj_length;
            let obj = Vec::from(&objects[start_idx..start_idx+obj_length]);
            // get color from color-mode
            let color = get_obj_color(&obj, 1.);
            canvas.set_stroke_style(&color);
            canvas.set_fill_style(&color);

            // DISPLAY OBJECTs
            if self.config.obj_families[family.id].is_displaying_objects {
                let (x, y) = (obj[1], obj[2]);
                canvas.draw_circle( (x, y), r, is_filled )
            }
            // DISPLAY POSITION VECTORs
            if self.config.obj_families[family.id].is_displaying_pos_vec {
                let (x, y) = (obj[1], obj[2]);
                canvas.draw_line((x, y), (0., 0.));
            }
            // DISPLAY VELOCITY VECTORs
            if self.config.obj_families[family.id].is_displaying_vel_vec {
                let (x, y, u, v) = (obj[1], obj[2], obj[3], obj[4]);
                let z = (u.powf(2.) + v.powf(2.)).powf(-0.5) / 5.; // TODO make configurable
                canvas.draw_line((x, y), (x+u*z, y+v*z));
            }

            // DISPLAY ACCELERATION VECTOR
            // if self.config.obj_families[family.id].is_displaying_acc_vec {
            //     // TODO
            // }

        }

        // DISPLAY OBJECT CENTER-OF-MASS
        if self.config.obj_families[family.id].is_displaying_center_of_mass {
            self.display_center_of_mass(&family, canvas_id, &engine);
        }

        // DISPLAY OBJECT CENTER-OF-MOMENTUM
        // if self.config.obj_families[family.id].is_displaying_center_of_momentum {
        //     self.display_center_of_momentum(&family, canvas_id);
        // }

        // DISPLAY OBJECT TAILS
        let tail_variant = &self.config.obj_families[family.id].tail_variant;
        match tail_variant {
            ObjTailVariant::Line => {     // LINE TAILS
                self.display_line_tails(&family, states, canvas_id, &engine);
            }, ObjTailVariant::Area => {  // AREA TAILS
                self.display_area_tails(&family, states, canvas_id, &engine);
            }, _ => {
            }
        }
    }

    // pub fn display_center_of_momentum(&mut self, family: &ObjFamily, canvas_id: usize) {

    //     let canvas = &mut self.canvases[canvas_id];
    //     let r = 10.;

    //     let mut center_of_momentum = (0., 0.);
    //     for obj_id in 0..family.nr_of_objects {
    //         let start_idx = obj_id * family.obj_length;
    //         let obj = &family.objects[start_idx..start_idx+family.obj_length];
    //         center_of_momentum.0 += obj[0] * obj[3];
    //         center_of_momentum.1 += obj[0] * obj[4];
    //     };

    //     canvas.draw_line(center_of_momentum, r, true);
    // }

    pub fn display_center_of_mass(
        &mut self, 
        family: &ObjFamily, 
        canvas_id: usize,
        engine: &Engine,
    ) {

        let canvas = &mut self.canvases[canvas_id];
        let r = 0.01;
        canvas.set_stroke_style("red");
        canvas.set_fill_style("red");

        let mut center_of_mass = (0., 0.);
        let mut total_mass = 0.;

        let nr_of_objects = engine.config.obj_families[family.id].family_size;
        let obj_length = engine.config.obj_families[family.id].obj_length;

        for obj_id in 0..nr_of_objects {
            let start_idx = obj_id * obj_length;
            let obj = &family.objects[start_idx..start_idx+obj_length];
            center_of_mass.0 += obj[1];
            center_of_mass.1 += obj[2];
            total_mass += obj[0];
        };
        center_of_mass.0 /= total_mass;
        center_of_mass.1 /= total_mass;

        canvas.draw_circle(center_of_mass, r, true);
    }

    pub fn display_line_tails(
        &mut self,
        family: &ObjFamily,
        states: &Vec<State>,
        canvas_id: usize,
        engine: &Engine,
    ) {
        let tail_length = 100; // TODO make configurable
        let tail_width = 2.; // TODO make configurable

        let nr_of_objects = engine.config.obj_families[family.id].family_size;
        let obj_length = engine.config.obj_families[family.id].obj_length;

        // SETUP CANVAS
        let canvas = &mut self.canvases[canvas_id];
        canvas.set_line_width(tail_width);

        // SETUP COLOR
        let obj_color_mode = &self.config.obj_families[family.id].color_mode;
        let get_obj_color = match obj_color_mode {
            ObjColorMode::Default     => object::color_mode::get_obj_color_default,
            ObjColorMode::Mass        => object::color_mode::get_obj_color_from_mass,
            ObjColorMode::HSLVelocity => object::color_mode::get_obj_color_from_velocity_angle, 
            ObjColorMode::HSLPosition => object::color_mode::get_obj_color_from_position_angle, 
            ObjColorMode::Speed       => object::color_mode::get_obj_color_from_speed,
            ObjColorMode::Distance    => object::color_mode::get_obj_color_from_distance, // NOTE from origin
            ObjColorMode::Charge      => object::color_mode::get_obj_color_from_charge,
        };

        let iterator = 0..usize::min(tail_length, self.frame_idx);
        for tail_step_id in iterator.rev() {

            for obj_id in 0..nr_of_objects {
                let start_idx = obj_id*obj_length;

                let state = &states[self.frame_idx - tail_step_id];
                let obj = &state.obj_families[family.id].objects[start_idx..start_idx+obj_length];
                let previous_state = &states[self.frame_idx - tail_step_id - 1];
                let previous_obj = &previous_state.obj_families[family.id].objects[start_idx..start_idx+obj_length];

                let (x1, y1) = (previous_obj[1], previous_obj[2]);
                let (x2, y2) = (obj[1], obj[2]);

                // setup color
                let alpha = 1. - tail_step_id as f64 / tail_length as f64;
                let color = get_obj_color( &Vec::from(obj), alpha );
                canvas.set_stroke_style(&color);
                canvas.set_fill_style(&color);

                canvas.draw_line( (x1, y1), (x2, y2) );

            }  // TODO apply alpha   (from input?)
        }
        canvas.reset_line_width();
    }

    pub fn display_area_tails(
        &mut self,
        family: &ObjFamily,
        states: &Vec<State>,
        canvas_id: usize,
        engine: &Engine,
    ) {
        let canvas = &mut self.canvases[canvas_id];
        let tail_length = 200; // TODO make configurable

        // setup color
        let obj_color_mode = &self.config.obj_families[family.id].color_mode;
        let get_obj_color = match obj_color_mode {
            ObjColorMode::Default     => object::color_mode::get_obj_color_default,
            ObjColorMode::Mass        => object::color_mode::get_obj_color_from_mass,
            ObjColorMode::HSLVelocity => object::color_mode::get_obj_color_from_velocity_angle, 
            ObjColorMode::HSLPosition => object::color_mode::get_obj_color_from_position_angle, 
            ObjColorMode::Speed       => object::color_mode::get_obj_color_from_speed,
            ObjColorMode::Distance    => object::color_mode::get_obj_color_from_distance, // NOTE from origin
            ObjColorMode::Charge      => object::color_mode::get_obj_color_from_charge,
        };

        let nr_of_objects = engine.config.obj_families[family.id].family_size;
        let obj_length = engine.config.obj_families[family.id].obj_length;

        let iterator = 0..usize::min(tail_length, self.frame_idx);
        for tail_step_id in iterator.rev() {

            for obj_id in 0..nr_of_objects {
                let start_idx = obj_id*obj_length;

                let state = &states[self.frame_idx - tail_step_id];
                let obj = &state.obj_families[family.id].objects[start_idx..start_idx+obj_length];
                let previous_state = &states[self.frame_idx - tail_step_id - 1];
                let previous_obj = &previous_state.obj_families[family.id].objects[start_idx..start_idx+obj_length];

                let (x1, y1) = (previous_obj[1], previous_obj[2]);
                let (x2, y2) = (obj[1], obj[2]);
                let (x3, y3) = (0., 0.);

                // setup color
                let alpha = 1. - tail_step_id as f64 / tail_length as f64;
                let color = get_obj_color( &Vec::from(obj), alpha );
                canvas.set_stroke_style(&color);
                canvas.set_fill_style(&color);

                canvas.draw_triangle( (x1, y1), (x2, y2), (x3, y3) )

            }  // TODO apply alpha   (from input?)
        }
    }

    pub fn display_field(
        &mut self, 
        field: &Field, 
        _states: &Vec<State>,
        canvas_id: usize,
        engine: &Engine,
    ) {

        let canvas = &mut self.canvases[canvas_id];

        let dimensions = &engine.config.fields[field.id].dimensions;
        for idx in 0..dimensions[0] {
            for jdx in 0..dimensions[1] {  // TODO handle z ?
                let cell = &field.entries[jdx*dimensions[0]+idx];
                let cell = *cell as i32;

                let color = match cell {
                    -1 => "black",
                    1 => "white",
                    _ => ""
                };

                let x = (idx as f64 / dimensions[0] as f64)*2.-1.;
                let y = (jdx as f64 / dimensions[1] as f64)*2.-1.;
                let w = 1. / dimensions[0] as f64;
                let h = 1. / dimensions[1] as f64;
                canvas.set_fill_style(&color);
                canvas.fill_rect((x, y), w, h);
            }
        }

        // for (cell_idx, cell) in field.entries.iter().enumerate() {
        //     match field.variant {
        //         FieldVariant::Ising => {
        //         }, _ => {}
        //     }
        //     // match self.sim_id {
        //     // }
        // }

        // let canvas = &mut self.canvases[canvas_id];
        // console::log("aaa");

        // let (x, y, r) = (0., 0., 0.1);
        // canvas.draw_circle((x,y), r, true);

    }

    pub fn display_hud(
        &mut self, 
        engine: &Engine,
    ) {
        let canvas = &mut self.canvases[0];
        canvas.set_font("36px sans-serif");
        canvas.set_stroke_style("white");
        canvas.set_fill_style("white");

        let frame_idx = format!("{}", self.frame_idx);
        canvas.fill_text(&frame_idx, 20., 50.);

        let iter_idx = format!("{}", engine.config.iter_idx); 
        canvas.fill_text(&iter_idx, 20., 100.);

    }

    pub fn reset(&mut self) {
        self.frame_idx = 0;  // TODO this does not reset engine (?)
        for canvas in self.canvases.iter_mut() { canvas.clear(); }
    }
}

