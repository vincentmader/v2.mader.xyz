#![allow(non_snake_case)]


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

// use mxyz_engine::config::field::FieldEngineConfig;
// use mxyz_engine::integrator::field::cell_auto::apply_periodic_bounds;


pub struct Renderer {

    sim_id:            String,
    pub canvases:          Vec<Canvas>,
    pub config:        config::RendererConfig,

}

impl Renderer {

    pub fn new(sim_id: &str) -> Self {
        Renderer {
            sim_id:    String::from(sim_id),
            canvases:  Vec::new(),
            config:    config::RendererConfig::new(sim_id),
            // TODO tail pre-computes (?)
        }
    }

    pub fn init(&mut self, engine: &Engine) {
        self.config.init(&engine);
        // initialize canvas   (TODO generalize)
        let canvas = Canvas::new("canvas_main");
        self.canvases.push(canvas);
        // initialize user inputs (TODO)
        let doc = mxyz_utils::dom::document();
        self.init_button_menu_1(&doc, );
        let initial_state = &engine.states[self.config.frame_idx];
        self.init_button_menu_2(&doc, &initial_state);
    }

    pub fn display(&mut self, engine: &Engine) {
        let states = &engine.states;
        // STATE SETUP
        let current_state = &states[self.config.frame_idx];
        let fields        = &current_state.fields;
        let families      = &current_state.obj_families;
        // CANVAS SETUP
        let canvas_id     = 0;  // todo: get id 
        let canvas        = &mut self.canvases[canvas_id];
        if self.config.is_clearing_canvas { canvas.clear(); }

        // TODO put somewhere else
        // let canvas = &mut self.canvases[canvas_id];
        // DISPLAY FIELD
        // let dimensions = Vec::from([40, 40]);
        // for row_idx in 0..dimensions[0] {
        //     for col_idx in 0..dimensions[1] {

        //         let x = (2.*(col_idx as f64 + 0.5) / dimensions[0] as f64) - 1.;  // TODO zoom
        //         let y = (2.*(row_idx as f64 + 0.5) / dimensions[1] as f64) - 1.;
        //         let (m, u, v, q) = (1., 0., 0., 1.);
        //         let mut force = Vec::from([0., 0.]);

        //         for family in families.iter() {
        //             use mxyz_engine::state::object::variant::ObjVariant;
        //             match engine.config.obj_families[family.id].obj_variant {
        //                 ObjVariant::Particle => { continue; },
        //                 _ => {}
        //             }

        //             let nr_of_objects = engine.config.obj_families[family.id].family_size;
        //             let obj_length = &engine.config.obj_families[family.id].obj_attributes.len();
        //             let objects = &family.objects;
        //             for obj_id in 0..nr_of_objects {
        //                 let obj = Vec::from(&objects[obj_id*obj_length..(obj_id+1)*obj_length]);

        //                 use mxyz_engine::interaction::object::object::forces as obj_obj_forces;
        //                 let eps = 0.;

        //                 let force_getter = match self.sim_id.as_str() {
        //                     "lennard-jones" => obj_obj_forces::lennard_jones::force,
        //                     "charge-interaction" => obj_obj_forces::coulomb::force,
        //                     _ => obj_obj_forces::newtonian_gravity::force,
        //                 };

        //                 let f = force_getter(
        //                     &[m, x, y, u, v, q], &obj, eps,
        //                 );
        //                 force[0] += f[0];
        //                 force[1] += f[1];
        //             }
        //         }
        //         // let force = cell;
        //         // let x = (2.*(col_idx as f64 + 0.5) / dimensions[0] as f64) - 1.;  // TODO zoom
        //         // let y = (2.*(row_idx as f64 + 0.5) / dimensions[1] as f64) - 1.;
        //         let norm = (force[0].powf(2.) + force[1].powf(2.)).sqrt();
        //         let from = (x, y);
        //         let to = (x + force[0] / norm/dimensions[0] as f64, y + force[1] / norm/dimensions[1] as f64);
        //         let radius = 0.001;
        //         let max_force = 10.;  // TODO make slider
        //         let r = 255. * norm / max_force;
        //         let (g, b) = (r, r);
        //         let color = format!("rgb({}, {}, {})", r, g, b);
        //         canvas.set_stroke_style(&color);
        //         canvas.set_fill_style(&color);
        //         canvas.draw_line(from, to);
        //         canvas.draw_circle(to, radius, true);
        //     }
        // }

        // DISPLAY FIELDS
        for field in fields.iter() {
            self.display_field(field, states, canvas_id, &engine);
        }
        // DISPLAY OBJECT FAMILIES
        for family in families.iter() {
            self.display_objects(family, states, canvas_id, &engine);
        }
        // DISPLAY HUD
        self.display_info_textfields(&engine);
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

        let r = self.config.obj_families[family.id].obj_drawing_radius;
        let is_filled = self.config.obj_families[family.id].obj_is_filled;

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
        // DISPLAY OBJECT CENTER-OF-MOMENTUM   TODO
            // if self.config.obj_families[family.id].is_displaying_center_of_momentum {
            //     self.display_center_of_momentum(&family, canvas_id, &engine);
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

    // pub fn display_center_of_momentum(
    //     &mut self, 
    //     family: &ObjFamily, 
    //     canvas_id: usize,
    //     engine: &Engine,
    // ) {
    //     let canvas = &mut self.canvases[canvas_id];

    //     let conf = &engine.config.obj_families[family.id];
    //     let obj_length = conf.obj_length;

    //     let mut center_of_momentum = (0., 0.);
    //     for obj_id in 0..conf.family_size {
    //         let start_idx = obj_id * obj_length;
    //         let obj = &family.objects[start_idx..start_idx+obj_length];
    //         center_of_momentum.0 += obj[0] * obj[3];
    //         center_of_momentum.1 += obj[0] * obj[4];
    //     };

    //     let from = (0., 0.);
    //     let to = (from.0 + center_of_momentum.0, from.1 + center_of_momentum.1);
    //     canvas.draw_line(from, to);
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
        // get info from engine-config
        let nr_of_objects = engine.config.obj_families[family.id].family_size;
        let obj_length = engine.config.obj_families[family.id].obj_length;
        // get info from renderer-config
        let tail_length = self.config.obj_families[family.id].tail_length;
        let tail_width = self.config.obj_families[family.id].tail_width;

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

        let frame_idx = self.config.frame_idx;
        for tail_step_id in (0..usize::min(tail_length, frame_idx)).rev() {

            for obj_id in 0..nr_of_objects {
                let start_idx = obj_id*obj_length;

                let state           = &states[frame_idx-tail_step_id];
                let obj             = &state.obj_families[family.id].objects[start_idx..start_idx+obj_length];
                let previous_state  = &states[frame_idx-tail_step_id-1];
                let previous_obj    = &previous_state.obj_families[family.id].objects[start_idx..start_idx+obj_length];

                let (x1, y1) = (previous_obj[1], previous_obj[2]);
                let (x2, y2) = (obj[1], obj[2]);

                // setup color
                let alpha = 1. - tail_step_id as f64 / tail_length as f64;
                let color = get_obj_color( &Vec::from(obj), alpha );
                canvas.set_stroke_style(&color);
                canvas.set_fill_style(&color);
                // draw tail
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
        // get info from engine-config
        let nr_of_objects = engine.config.obj_families[family.id].family_size;
        let obj_length = engine.config.obj_families[family.id].obj_length;
        // get info from renderer-config
        let tail_length = self.config.obj_families[family.id].tail_length;

        // SETUP CANVAS
        let canvas = &mut self.canvases[canvas_id];

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

        let iterator = 0..usize::min(tail_length, self.config.frame_idx);
        for tail_step_id in iterator.rev() {

            for obj_id in 0..nr_of_objects {
                let start_idx = obj_id*obj_length;

                let state = &states[self.config.frame_idx - tail_step_id];
                let obj = &state.obj_families[family.id].objects[start_idx..start_idx+obj_length];
                let previous_state = &states[self.config.frame_idx - tail_step_id - 1];
                let previous_obj = &previous_state.obj_families[family.id].objects[start_idx..start_idx+obj_length];

                let (x1, y1) = (previous_obj[1], previous_obj[2]);
                let (x2, y2) = (obj[1], obj[2]);
                let (x3, y3) = (0., 0.);

                // setup color
                let alpha = 1. - tail_step_id as f64 / tail_length as f64;
                let color = get_obj_color( &Vec::from(obj), alpha );
                canvas.set_stroke_style(&color);
                canvas.set_fill_style(&color);
                // draw tail
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

        for x_idx in 0..dimensions[0] {
            for y_idx in 0..dimensions[1] {

        // for dimension_idx in 0..dimensions.len() {
        //     for idx in 0..dimensions[dimension_idx] {
                let cell = field.entries[y_idx*dimensions[0]+x_idx];


        //         // let force = cell;
        //         // let x = (2.*(col_idx as f64 + 0.5) / dimensions[0] as f64) - 1.;  // TODO zoom
        //         // let y = (2.*(row_idx as f64 + 0.5) / dimensions[1] as f64) - 1.;
        //         // let norm = (force[0].powf(2.) + force[1].powf(2.)).sqrt();
        //         // let from = (x, y);
        //         // let to = (x + force[0] / norm/dimensions[0] as f64, y + force[1] / norm/dimensions[1] as f64);
        //         // let radius = 0.001;
        //         // let max_force = 10.;  // TODO make slider
        //         // let r = 255. * norm / max_force;
        //         // let (g, b) = (r, r);
        //         // let color = format!("rgb({}, {}, {})", r, g, b);
        //         // canvas.set_stroke_style(&color);
        //         // canvas.set_fill_style(&color);
        //         // canvas.draw_line(from, to);
        //         // canvas.draw_circle(to, radius, true);






        //         // match self.sim_id {
        //         //     "game-of-life" => {

        //         //     }, "ising" => {

        //         //     }, _ => {

        //         //     }
        //         // }

                let color = if cell == 1. { "white" } 
                    else if cell == 2. { "red" } 
                    else { "black" };

                let s = match self.sim_id.as_str() {
                    "game-of-life" => 1.,
                    _ => 0.5
                };
                let x = ((x_idx as f64 + (1.-s)/2.) / dimensions[0] as f64)*2.-1.;
                let y = ((y_idx as f64 + (1.-s)/2.) / dimensions[1] as f64)*2.-1.;
                let w = s * 2. / dimensions[0] as f64;
                let h = s * 2. / dimensions[1] as f64;
                canvas.set_fill_style(&color);
                canvas.fill_rect((x, y), w, h);

        //         // let (x, y) = (
        //         //     (idx as f64+0.2) / dimensions[0] as f64 * canvas.dimensions.0, 
        //         //     (jdx as f64+0.2) / dimensions[1] as f64 * canvas.dimensions.1, 
        //         // );
        //         // use mxyz_engine::integrator::field::cell_auto::get_nr_of_neighbors;
        //         // let nr_of_neighbors = get_nr_of_neighbors(
        //         //     field, &engine.config.fields[field.id], idx, jdx, 0
        //         // );
        //         // let next = match nr_of_neighbors {
        //         //     2 => if cell == 1. {1.} else {0.}, 3 => 1., _ => 0.
        //         // };
        //         // canvas.set_font("18px sans-serif");
        //         // canvas.set_stroke_style("green");
        //         // canvas.set_fill_style("green");
        //         // if nr_of_neighbors != 0 {
        //         //     // canvas.fill_text(&format!("{}", nr_of_neighbors), x, y);
        //         //     // canvas.fill_text(&format!("{}", next), x, y);
        //         //     // canvas.fill_text(&format!("({}, {}):  {} -> {}", jdx, idx, nr_of_neighbors, next), x, y);
        //         //     canvas.fill_text(&format!("({}, {}):  {}", jdx, idx, nr_of_neighbors), x, y);
        //         // }
            }
        }
    }

    pub fn display_info_textfields(
        &mut self, 
        engine: &Engine,
    ) {
        let doc = mxyz_utils::dom::document();

        let textfield = doc.get_element_by_id("textfield_iter_idx").unwrap();
        textfield.set_inner_html(&format!("iter: {}", engine.config.iter_idx));
        let textfield = doc.get_element_by_id("textfield_frame_idx").unwrap();
        textfield.set_inner_html(&format!("frame: {}", self.config.frame_idx));
    }

    pub fn display_hud(
        &mut self, 
        _engine: &Engine,
    ) {
        let canvas = &mut self.canvases[0];
        canvas.set_font("36px sans-serif");
        canvas.set_stroke_style("white");
        canvas.set_fill_style("white");

        let frame_idx = format!("{}", self.config.frame_idx);
        canvas.fill_text(&frame_idx, 20., 50.);
        // let iter_idx = format!("{}", engine.config.iter_idx); 
        // canvas.fill_text(&iter_idx, 20., 100.);

    }

    pub fn reset(&mut self) {
        self.config.frame_idx = 0;  // TODO this does not reset engine (?)
        for canvas in self.canvases.iter_mut() { canvas.clear(); }
    }
}

