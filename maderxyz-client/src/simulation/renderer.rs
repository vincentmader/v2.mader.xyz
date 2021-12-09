
use std::cmp;
use std::f64::consts::TAU;

use maderxyz_numerics::state::State;
use maderxyz_numerics::state::ObjectFamily;
use maderxyz_numerics::state::object_family::TailVariant;
use maderxyz_numerics::state::ObjectType;
use maderxyz_numerics::state::ObjectAttribute;
use maderxyz_numerics::state::Field;

use crate::utils::dom;
use crate::utils::dom::Canvas;
use crate::utils::dom::console_log;


pub struct Renderer {
    page_id: String,
    canvas: Canvas,  // TODO multiple?
    pub frame_idx: usize,
    pub is_paused: bool,
    // TODO move elsewhere (for each obj family + total)
    pub object_color_mode: ObjectColorMode,
    pub display_tails: bool,
}
impl Renderer {
    pub fn new(page_id: &str) -> Self {

        let canvas_centered = is_canvas_centered(page_id);
        let mut canvas = Canvas::new("canvas_main", canvas_centered);
        match page_id {
            "3body-broucke" => {canvas.zoom /= 2.5;},
            _ => {}
        }

        let (frame_idx, is_paused) = (0, false);
        Renderer { 
            page_id: String::from(page_id),
            canvas,
            frame_idx,
            is_paused,
            object_color_mode: ObjectColorMode::Default,
            display_tails: false,
        }
    }
    pub fn init(&mut self) {

    }
    pub fn display(&mut self, states: &Vec<State>) {
        self.display_menu(states);
        self.canvas.clear();
        self.canvas.set_fill_style("white");  // for dev

        self.display_fields(states);
        self.display_objects(states);

        if !self.is_paused && self.frame_idx+1 < states.len() { self.frame_idx += 1; } 
        // if self.frame_idx >= states.len() { return (); }
    }
    fn display_fields(&mut self, states: &Vec<State>) {
        let current_state   = &states[self.frame_idx];
        let fields          = &current_state.fields;
        for field in fields.iter() {
            self.display_field(&states, field);
        }

    }
    fn display_objects(&mut self, states: &Vec<State>) {
        let current_state   = &states[self.frame_idx];
        let object_families = &current_state.object_families;
        for object_family in object_families.iter() {
            self.display_object_family(&states, object_family);
        } 
    }
    fn display_field(&mut self, states: &Vec<State>, field: &Field) {
        let grid_width: usize = field.dimensions.0;
        let grid_height: usize = field.dimensions.1; // x first!

        match self.page_id.as_str() {
            "diffusion" => {},
            _ => {}
        }

        for row_idx in 0..grid_height {
            for col_idx in 0..grid_width {
                let density = &field.cells[row_idx*grid_height+col_idx][0];
                let z = 0.97;
                let (w, h) = (2.*z / grid_width as f64, 2.*z / grid_height as f64);
                let (x, y) = (
                    2. * col_idx as f64 / grid_height as f64 - 1.,
                    2. * row_idx as f64 / grid_width as f64 - 1.
                );
                let (r, g, b) = (255, 255, 255);
                let alpha = density;
                let color = format!("rgba({}, {}, {}, {})", r, g, b, alpha);
                self.canvas.set_fill_style(&color);
                self.canvas.fill_rect((x, y), w, h);
                // self.canvas.draw_circle((x, y), w/2., true);
            }
        }
    }
    fn display_object_family(
        &mut self, 
        states: &Vec<State>, 
        object_family: &ObjectFamily,
        // object_color_mode: &ObjectColorMode,
    ) {
        let drawing_radius: f64 = match object_family.object_type {
            ObjectType::Static => 0.05,    // 0.025
            ObjectType::Body => 0.03,      // 0.015
            ObjectType::Particle => 0.02,  // 0.0025
        };

        let family_idx = object_family.id;
        let objects = &object_family.objects;
        let nr_of_objects = objects.len();

        let iteration_step = states.len() - 1;
        let particles_carry_charge = object_family.attributes.contains(&ObjectAttribute::Charge);

        let object_color_mode = &self.object_color_mode;
        let get_object_color = match object_color_mode {
            ObjectColorMode::HSLVelocity => get_object_color_from_velocity_angle, 
            ObjectColorMode::HSLPosition => get_object_color_from_position_angle, 
            ObjectColorMode::Speed => get_object_color_from_speed,
            ObjectColorMode::Distance => get_object_color_from_distance, // NOTE from origin
            ObjectColorMode::Default => get_object_color_default,
        };

        // draw tails
        // ==========
        if self.display_tails {
            let tail_length = cmp::min(object_family.tail_length, self.frame_idx);

            self.canvas.context.set_line_width(1.);
            for tail_idx in 0..tail_length {
                let idx = self.frame_idx - tail_length + tail_idx;
                let previous_idx = cmp::max(0, idx as i32 - 1) as usize;
                let alpha = (tail_idx as f64) / (tail_length as f64);
                // loop over objects
                for object_idx in 0..nr_of_objects {
                    let object = &states[idx].object_families[family_idx].objects[object_idx];
                    let previous = &states[previous_idx].object_families[family_idx].objects[object_idx];
                    // draw
                    let color = get_object_color(&object, alpha);
                    self.canvas.set_stroke_style(&color);

                    self.canvas.draw_line(
                        (previous[1], previous[2]), 
                        (object[1], object[2]),
                    );  
                    if matches!(object_family.tail_variant, TailVariant::Default) {

                    } else if matches!(object_family.tail_variant, TailVariant::Area) {
                        self.canvas.set_fill_style(&color);
                        self.canvas.draw_triangle(
                            (0., 0.),
                            (previous[1], previous[2]), 
                            (object[1], object[2]),
                        )
                    }
                }
            }
        }

        // draw velocity vectors
        // for object_idx in 0..nr_of_objects {
        //     let object = &states[self.frame_idx].object_families[family_idx].objects[object_idx];
        //     let previous = &states[cmp::max(1, self.frame_idx)-1].object_families[family_idx].objects[object_idx]; // TODO why not max?
        //     // draw
        //     let color = get_object_color(&object, 1.);
        //     self.canvas.set_stroke_style(&color);

        //     let dx = object[1] - previous[1];
        //     let dy = object[2] - previous[2];
        //     let k = 20.;
 
        //     self.canvas.draw_line(
        //         (previous[1], previous[2]), 
        //         (previous[1] + k*dx, previous[2] + k*dy),
        //     );
        // }

        // draw objects
        // ==========
        for object in objects.iter() {
            // load object state
            let m = object[0];
            let x = object[1];
            let y = object[2];
            let u = object[3];
            let v = object[4];
            // handle charge
            let mut q = 0.;
            if particles_carry_charge { q = object[5]; }
            // set color
            let mut color = get_object_color(&object, 1.);
            match object_family.object_type {
                ObjectType::Static => {color = String::from("white");},
                _ => {}
            }
            // draw
            self.canvas.set_fill_style(&color);
            self.canvas.set_stroke_style(&color);
            self.canvas.context.set_line_width(1.);  // TODO
            self.canvas.draw_circle((x, y), drawing_radius, true);
        }
    }
    pub fn display_menu(&self, states: &Vec<State>) {  // TODO ?
        let iteration_step = states.len();
        let current_state = &states[iteration_step-1];

        let elm_frame = dom::document().get_element_by_id("display_frame_idx").unwrap();
        elm_frame.set_inner_html(&format!("frame idx: {}", self.frame_idx));

        let iter_idx = current_state.iteration_idx;
        let elm_iter = dom::document().get_element_by_id("display_iteration_idx").unwrap();
        elm_iter.set_inner_html(&format!("iteration idx: {}", iter_idx));
    }
}


pub fn get_hsl_from_vec(vec: [f64; 2], alpha: f64) -> String {
    let phi = vec[1].atan2(vec[0]) / TAU * 360.;
    let (h, s, l) = (phi, 100, 50);
    format!("hsla({}, {}%, {}%, {})", h, s, l, alpha)
}

pub fn get_unit_vec_from_angle(phi: f64) -> [f64; 2] {
    [phi.cos(), phi.sin()]
}

pub fn is_canvas_centered(page_id: &str) -> bool {
   match page_id {
       "charge-interaction" | "lennard-jones" => false,
       _ => true,
   }
}

pub enum ObjectColorMode {
    // Preset,
    HSLPosition,
    HSLVelocity,
    // Mass,
    Speed,
    Distance,
    // Charge,
    Default, // NOTE white
}

// TODO only return rgb values, apply alpha later! (from tail_idx)

fn get_object_color_from_velocity_angle(obj: &Vec<f64>, alpha: f64) -> String {
    get_hsl_from_vec([obj[3], obj[4]], alpha)
}
fn get_object_color_from_position_angle(obj: &Vec<f64>, alpha: f64) -> String {
    get_hsl_from_vec([obj[1], obj[2]], alpha)
}
fn get_object_color_from_speed(obj: &Vec<f64>, alpha: f64) -> String {
    const MAX_SPEED: f64 = 1.5;
    let u = obj[3];
    let v = obj[4];
    let speed = (u.powf(2.) + v.powf(2.)).sqrt();
    let foo = f64::min(1., speed / MAX_SPEED) * 255.;

    // TODO generalize gradients
    let r = foo;
    let g = 255. - (255. * (foo-127.).abs()/128.);  
    let b = 255. - foo;
    format!("rgba({}, {}, {}, {})", r, g, b, alpha)
}
fn get_object_color_from_distance(obj: &Vec<f64>, alpha: f64) -> String {
    const MAX_DIST: f64 = 1.;
    let x = obj[1];
    let y = obj[2];
    let dist = (x.powf(2.) + y.powf(2.)).sqrt();
    let foo = f64::min(1., dist / MAX_DIST) * 255.;

    let r = foo;
    let r = 255. - r; // flip blue & red
    let g = 255. - (255. * (r-127.).abs()/128.);
    let b = 255. - r;
    format!("rgba({}, {}, {}, {})", r, g, b, alpha)
}
fn get_object_color_default(obj: &Vec<f64>, alpha: f64) -> String {
    format!("rgba(255, 255, 255, {})", alpha)
}

