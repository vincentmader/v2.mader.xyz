
use std::cmp;
use std::f64::consts::TAU;

use maderxyz_numerics::state::State;
use maderxyz_numerics::state::ObjectFamily;
use maderxyz_numerics::state::ObjectType;
use maderxyz_numerics::state::ObjectAttribute;
use maderxyz_numerics::state::Field;

use crate::utils::dom;
use crate::utils::dom::Canvas;
use crate::utils::dom::console_log;


pub struct Renderer {
    page_id: String,
    canvas: Canvas,  // TODO multiple?
    frame_idx: usize,
    is_paused: bool,
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
        }
    }
    pub fn init(&mut self) {

        // let a = dom::document().get_element_by_id("main_title").unwrap();
        // a.set_inner_html(&self.title);

        // TODO add button handlers (-> to engine?)
        // fn pause_handler(&mut self) {self.pause()}
        // let pause_handler = || self.pause();
        // dom::add_button_to_menu("pause", pause_handler); 
    }
    pub fn display(&mut self, states: &Vec<State>) {
        if self.is_paused { return (); }
        self.canvas.clear();
        self.canvas.set_fill_style("white");  // for dev

        self.display_fields(states);
        self.display_objects(states);
        self.display_menu(states);

        self.frame_idx += 1;
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
        let GRID_WIDTH: usize = field.dimensions.0;
        let GRID_HEIGHT: usize = field.dimensions.1; // x first!

        match self.page_id.as_str() {
            "diffusion" => {},
            _ => {}
        }

        for row_idx in 0..GRID_HEIGHT {
            for col_idx in 0..GRID_WIDTH {
                let density = &field.cells[row_idx*GRID_HEIGHT+col_idx][0];
                let z = 0.97;
                let (w, h) = (2.*z / GRID_WIDTH as f64, 2.*z / GRID_HEIGHT as f64);
                let (x, y) = (
                    2. * col_idx as f64 / GRID_HEIGHT as f64 - 1.,
                    2. * row_idx as f64 / GRID_WIDTH as f64 - 1.
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
    fn display_object_family(&mut self, states: &Vec<State>, object_family: &ObjectFamily) {

        let family_idx = object_family.id;
        let objects = &object_family.objects;
        let nr_of_objects = objects.len();

        let iteration_step = states.len() - 1;
        let particles_carry_charge = object_family.attributes.contains(&ObjectAttribute::Charge);

        let object_color_mode = ObjectColorMode::HSLVelocity;
        let get_object_color = match object_color_mode {
            ObjectColorMode::HSLVelocity => get_object_color_from_velocity_angle, 
            ObjectColorMode::HSLPosition => get_object_color_from_position_angle, 
            ObjectColorMode::Speed => get_object_color_from_speed,
            ObjectColorMode::Distance => get_object_color_from_distance, // NOTE from origin
        };

        // draw tails
        // ==========
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
                let color = get_object_color(&object);

                self.canvas.set_stroke_style(&color);
                self.canvas.draw_line(
                    (previous[1], previous[2]), 
                    (object[1], object[2]),
                );  

                // self.canvas.set_fill_style(&color);
                // self.canvas.draw_triangle(
                //     (0., 0.),
                //     (previous[1], previous[2]), 
                //     (object[1], object[2]),
                // )
            }
        }

        // draw objects
        // ==========
        let drawing_radius: f64 = match object_family.object_type {
            ObjectType::Static => 0.025,
            ObjectType::Body => 0.015,
            ObjectType::Particle => 0.0025,
        };
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
            let mut color = get_object_color(&object);
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
    fn pause(&mut self) {
        self.is_paused = !self.is_paused;
    }
}


pub fn get_hsl_from_vec(vec: [f64; 2]) -> String {
    let phi = vec[1].atan2(vec[0]) / TAU * 360.;
    let (h, s, l) = (phi, 100, 50);
    format!("hsl({}, {}%, {}%)", h, s, l)
}

pub fn get_unit_vec_from_angle(phi: f64) -> [f64; 2] {
    [phi.cos(), phi.sin()]
}

pub fn is_canvas_centered(page_id: &str) -> bool {
   match page_id {
       "charge-interaction" => false,
       _ => true,
   }
}

enum ObjectColorMode {
    // Preset,
    HSLPosition,
    HSLVelocity,
    // Mass,
    Speed,
    Distance,
    // Charge,
}

// TODO only return rgb values, apply alpha later! (from tail_idx)

fn get_object_color_from_velocity_angle(obj: &Vec<f64>) -> String {
    get_hsl_from_vec([obj[3], obj[4]])
}
fn get_object_color_from_position_angle(obj: &Vec<f64>) -> String {
    get_hsl_from_vec([obj[1], obj[2]])
}
fn get_object_color_from_speed(obj: &Vec<f64>) -> String {
    const MAX_SPEED: f64 = 1.5;
    let u = obj[3];
    let v = obj[4];
    let speed = (u.powf(2.) + v.powf(2.)).sqrt();
    let foo = f64::min(1., speed / MAX_SPEED) * 255.;

    // TODO generalize gradients
    let r = foo;
    let g = 255. - (255. * (foo-127.).abs()/128.);  
    let b = 255. - foo;
    format!("rgb({}, {}, {})", r, g, b)
}
fn get_object_color_from_distance(obj: &Vec<f64>) -> String {
    const MAX_DIST: f64 = 1.;
    let x = obj[1];
    let y = obj[2];
    let dist = (x.powf(2.) + y.powf(2.)).sqrt();
    let foo = f64::min(1., dist / MAX_DIST) * 255.;

    let r = foo;
    let r = 255. - r; // flip blue & red
    let g = 255. - (255. * (r-127.).abs()/128.);
    let b = 255. - r;
    format!("rgb({}, {}, {})", r, g, b)
}

