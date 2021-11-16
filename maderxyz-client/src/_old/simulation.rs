
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct Simulation {
    title: String,
}
#[wasm_bindgen]
impl Simulation {
    pub fn new(title: String) -> Simulation {
        Simulation {
            title
        }
    }
    pub fn init(&self) {

    }
    pub fn step(&self) {

    }
}






// // // SIMULATION
// // // ===================================================

// pub struct Simulation {

// }
// impl Simulation {
//     pub fn new() -> Simulation {
//         Simulation {

//         }
//     }
//     pub fn init(&self) {
//         // animation_loop();
//     }
//     pub fn step(&self) {
//         self.render();
//     }
//     pub fn render(&self) {

//     }
// }

// // pub fn animation_loop(f: fn(Simulation) -> ()) {

// // }

// // // use wasm_bindgen::JsCast;

// // // use crate::utils::alert;


// // // SIMULATION
// // // ===================================================

// // pub struct Simulation {
// //     engine: Engine,
// //     renderer: Renderer,
// // }
// // impl Simulation {
// //     pub fn new() -> Simulation {
// //         let engine = Engine::new();
// //         let renderer = Renderer::new();
// //         Simulation { engine, renderer }
// //     }
// //     pub fn init(&self) {
// //         self.engine.init(); 
// //         self.renderer.init();
// //         self.start()
// //     }
// //     pub fn start(&self) {
// //         let a = self.renderer.clone();
// //         let renderer_ref = Rc::new(RefCell::new(a));
// //         animation_loop(0, Rc::clone(&renderer_ref));
// //     }
// // }

// // // SOLVERS
// // // ===================================================

// // // pub enum SolverVariantRk { Rk2, Rk4 }
// // // pub enum SolverVariantEuler { Implicit, Explicit }
// // // pub struct Euler { variant: SolverVariantEuler }
// // // impl Euler {
// // //     pub fn new(variant: SolverVariantEuler) -> Euler {
// // //         Euler { variant }
// // //     }
// // //     pub fn init(&self) {
// // //         match &self.variant {
// // //             Implicit => {},
// // //             Explicit => {},
// // //             _ => {}
// // //         }
// // //     } 
// // // }
// // // pub struct LeapFrog {}
// // // pub struct RungeKutta { variant: SolverVariantRk }

// // // ENGINE
// // // ===================================================

// // pub struct Engine {
// //     // universe: Universe,
// // }
// // impl Engine {
// //     pub fn new() -> Engine {
// //         Engine { }
// //     }
// //     pub fn init(&self) {
// //         // TODO: ... let eu = Euler::new(SolverVariantEuler::Implicit);
// //     }
// //     // pub fn step(&self) {
// //     //     // TODO: access to state?  (not self.state)
// //     // }
// // }

// // // RENDERER
// // // ===================================================

// // use crate::dom::canvas;
// // use crate::dom::ctx;

// // #[derive(Clone)]
// // pub struct Renderer {
// //     cnv: web_sys::HtmlCanvasElement,
// //     ctx: web_sys::CanvasRenderingContext2d,
// // }
// // impl Renderer {
// //     fn new() -> Renderer {
// //         let cnv = canvas("canvas_main");
// //         let ctx = ctx(&cnv);
// //         Renderer { cnv, ctx }
// //     }
// //     fn init(&self) {
// //         // animation_loop(0);
    
// //         self.display();
// //     }
// //     fn display(&self) {
// //         let canvas_width = f64::from(self.cnv.width());
// //         let canvas_height = f64::from(self.cnv.height());
// //         // self.ctx.clear_rect(0., 0., canvas_width, canvas_height);
// //         // const PI: f64 = std::f64::consts::PI;
// //         // const TAU: f64 = 2.0 * PI;
// //         // let x0 = canvas_width / 2.0;
// //         // let y0 = canvas_height / 2.0;
// //         self.ctx.set_stroke_style(&JsValue::from_str("purple"));
// //         // self.ctx.set_fill_style(&JsValue::from_str("purple"));
// //         let r = 100.;
// //         // for body in &self.universe.bodies {
// //         // let x = body.location.x + x0;
// //         // let y = body.location.y + y0;
// //         self.ctx.begin_path();
// //         self.ctx.arc(canvas_width/2.0, canvas_height/2.0, r, 0.0, 2.*3.14159).unwrap();
// //         self.ctx.stroke();
// //         self.ctx.fill();
// //         // TODO: access to state?
// //     }
// // }

// // // ANIMATION LOOP
// // // ===================================================

// use std::cell::RefCell;
// use std::rc::Rc;

// use wasm_bindgen::prelude::*;

// use crate::dom::request_animation_frame;

// struct State {
//     state: [f64; 64],
// }
// impl State {
//     // pub fn write(&mut self, idx: usize, value: f64) {
//     //     self.state[idx] = value;
//     // }
//     pub fn read(&self, idx: usize) -> f64 {
//         self.state[idx] 
//     }
// }

// use crate::dom::canvas;
// use crate::dom::ctx;

// use crate::dom::body;

// pub fn animation_loop() {

//     let f = Rc::new(RefCell::new(None));
//     let g = f.clone();

//     let mut i = 0;
//     *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    
//     let cnv = canvas("canvas_main");
//     // let ctx = ctx(&cnv);

//     // let canvas_width = f64::from(cnv.width());
//     // let canvas_height = f64::from(cnv.height());

//     // // ctx.clear_rect(0., 0., canvas_width, canvas_height);

//     // // const PI: f64 = std::f64::consts::PI;
//     // // const TAU: f64 = 2.0 * PI;
//     // // let x0 = canvas_width / 2.0;
//     // // let y0 = canvas_height / 2.0;
//     // ctx.set_stroke_style(&JsValue::from_str("purple"));
//     // ctx.set_fill_style(&JsValue::from_str("purple"));
//     // let r = 3.;
//     // // for body in &self.universe.bodies {
//     // //    let x = body.location.x + x0;
//     // //    let y = body.location.y + y0;
//     // ctx.begin_path();
//     // // ctx.arc(x, y, r, 0.0, TAU).unwrap();
//     // ctx.arc(canvas_width/2.0, canvas_height/2.0, r, 0.0, 2.*3.14159).unwrap();
//     // ctx.stroke();
 

//         // Set the body's text content to how many times this
//         // requestAnimationFrame callback has fired.
//         i += 1;
//         let text = format!("requestAnimationFrame has been called {} times.", i);
//         body().set_text_content(Some(&text));

//         // Schedule ourself for another requestAnimationFrame callback.
//         request_animation_frame(f.borrow().as_ref().unwrap());
//     }) as Box<dyn FnMut()>));

//     request_animation_frame(g.borrow().as_ref().unwrap());
// }

// // fn animation_loop(
// //     // mut frame_idx: usize, 
// //     // input: Rc<RefCell<Renderer>>
// // ) {

// //     let mut frame_idx: usize = 0;

// //     let s = State {
// //         state: [
// //             0., 1., 0., 1., 0., 1., 0., 1.,
// //             0., 1., 0., 1., 0., 1., 0., 1.,
// //             0., 1., 0., 1., 0., 1., 0., 1.,
// //             0., 1., 0., 1., 0., 1., 0., 1.,
// //             0., 1., 0., 1., 0., 1., 0., 1.,
// //             0., 1., 0., 1., 0., 1., 0., 1.,
// //             0., 1., 0., 1., 0., 1., 0., 1.,
// //             0., 1., 0., 1., 0., 1., 0., 1.,
// //         ],
// //     };

// //     // let cnv = &renderer.cnv;
// //     // let ctx = &renderer.ctx;
// //     // let my_ref = input.borrow_mut();
// //     // let mut my_ref = input.borrow_mut();


// //     let f = Rc::new(RefCell::new(None));
// //     let g = f.clone();
// //     *g.borrow_mut() = Some(
// //         Closure::wrap(
// //             Box::new(
// //                 move || {
// //                     // let a = self.cnv;
// //                     // renderer.display();
// //                     // display(self.cnv);
// //                     // next_animation_frame(&self.cnv);
// //                     // my_ref.display();
// //                     frame_idx += 1;

// //                     let res = s.read(frame_idx);

// //                     let x = f64::from(res).cos();
// //                     let y = f64::from(res).sin();
// //                     let r = 30.;

// //                     let text = format!("{}: {}", frame_idx, res);


// //                     body().set_text_content(Some(&text));
// //                     // draw_point(x, y, r);

// //                     draw_point(1.,1.,1.);
// //                     // start();
// //                     // Renderer::display(&renderer) ;
// //                     // if frame_idx == 10 {alert("heeeee")}
// //                     request_animation_frame(
// //                         f.borrow().as_ref().unwrap()
// //                     );
// //                 }
// //             ) as Box<dyn FnMut()>
// //         )
// //     );
// //     request_animation_frame(
// //         g.borrow().as_ref().unwrap()
// //     );
// // }


// fn draw_point(
//     x: f64, 
//     y: f64, 
//     r: f64,
// ) {

//     let cnv = canvas("canvas_simulation");
//     // let ctx = ctx(&cnv);

//     // let canvas_width = f64::from(cnv.width());
//     // let canvas_height = f64::from(cnv.height());

//     // // ctx.clear_rect(0., 0., canvas_width, canvas_height);

//     // // const PI: f64 = std::f64::consts::PI;
//     // // const TAU: f64 = 2.0 * PI;
//     // // let x0 = canvas_width / 2.0;
//     // // let y0 = canvas_height / 2.0;
//     // ctx.set_stroke_style(&JsValue::from_str("purple"));
//     // ctx.set_fill_style(&JsValue::from_str("purple"));
//     // let r = 3.;
//     // // for body in &self.universe.bodies {
//     // //    let x = body.location.x + x0;
//     // //    let y = body.location.y + y0;
//     // ctx.begin_path();
//     // // ctx.arc(x, y, r, 0.0, TAU).unwrap();
//     // ctx.arc(canvas_width/2.0, canvas_height/2.0, r, 0.0, 2.*3.14159).unwrap();
//     // ctx.stroke();
//     // ctx.fill();

// }


// // // pub fn start() {
// // //     let simulation_id: u16 = 0; 
// // //     let sim = Simulation::new(); // TODO: simulation_id);
// // //     sim.init();
// // // }


// // // pub struct Renderer {
// // //     simulation_id: u16,
// // // }
// // // impl Renderer {
// // //     pub fn new(simulation_id: u16) -> Renderer {
// // //         Renderer {
// // //             simulation_id, cnv, ctx
// // //         }
// // //     }
// // //     pub fn init(&self) {
// // //         // while true {
// // //             self.render()
// // //         // }
// // //     }
// // //     pub fn render(&self) {

// // //         let canvas_width = f64::from(self.cnv.width());
// // //         let canvas_height = f64::from(self.cnv.height());

// // //         self.ctx.clear_rect(0., 0., canvas_width, canvas_height);

// // //         // const PI: f64 = std::f64::consts::PI;
// // //         // const TAU: f64 = 2.0 * PI;
// // //         // let x0 = canvas_width / 2.0;
// // //         // let y0 = canvas_height / 2.0;
// // //         self.ctx.set_stroke_style(&JsValue::from_str("purple"));
// // //         self.ctx.set_fill_style(&JsValue::from_str("purple"));
// // //         let r = 3.;
// // //         // for body in &self.universe.bodies {
// // //         //    let x = body.location.x + x0;
// // //         //    let y = body.location.y + y0;
// // //         self.ctx.begin_path();
// // //         // ctx.arc(x, y, r, 0.0, TAU).unwrap();
// // //         self.ctx.arc(canvas_width/2.0, canvas_height/2.0, r, 0.0, 2.*3.14159).unwrap();
// // //         self.ctx.stroke();
// // //         self.ctx.fill();
// // //         // }
// // //     }
// // // }




// // // use std::cell::RefCell;
// // // use std::cell::Cell;
// // // use std::rc::Rc;

// // // use wasm_bindgen::prelude::*;
// // // use wasm_bindgen::JsCast;

// // // // use crate::dom::Dom;
// // // use crate::universe::Universe;
// // // use crate::body::Body;
// // // use crate::quadtree::QTNode;
// // // use crate::utils::Vector2D;
// // // // use crate::maderxyz_simulations::nbody_solver::MoonSim;

// // // use crate::dom::canvas;
// // // use crate::dom::ctx;
// // // use crate::dom::request_animation_frame;


// // // // #[derive(Copy, Clone)]
// // // pub struct Simulation {
// // //     step_idx: u32,
// // //     universe: Universe,
// // // }

// // // impl Simulation {

// // //     pub fn new() -> Simulation {
// // //         let mut bodies: Vec<Body> = Vec::new();
// // //         let N = 2;
// // //         let R = 100.;
// // //         for i in 0..N {
// // //             let phi = f64::from(i) / f64::from(N) * 2. * 3.14159;
// // //             let body = Body::new(
// // //                 i, 1., 
// // //                 Vector2D::new(R * phi.cos(), R * phi.sin()), 
// // //                 Vector2D::new(0., 0.)
// // //             );
// // //             bodies.push(body);
// // //         }

// // //         let universe = Universe::new(bodies);
// // //         let step_idx = 0;

// // //         Simulation {
// // //             step_idx,
// // //             universe
// // //         }
// // //     }

// // //     pub fn init(self) {
// // //         self.animation_loop().unwrap();
// // //     }

// // //     fn animation_loop(self) -> Result<(), JsValue> {
// // //         let f = Rc::new(RefCell::new(None));
// // //         let g = f.clone();

// // //         *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
// // //             self.update();
// // //             request_animation_frame(f.borrow().as_ref().unwrap());
// // //         }) as Box<dyn FnMut()>));

// // //         request_animation_frame(g.borrow().as_ref().unwrap());
// // //         Ok(())
// // //     }

// // //     fn update(self) {
// // //         // self.update_system_state();
// // //         // self.update_canvas();
// // //         // self.step();
// // //         // self.draw();
// // //         self.step_idx += 1;
// // //     }

// // //     pub fn step(mut self) {

// // //         let mut bodies = self.universe.bodies; //.clone();
// // //         // let bodies2 = self.universe.bodies.clone();

// // //         for mut body in bodies.iter_mut() {

// // //             // let mass_thingies: Vec<QTNode> = Vec::new(); // TODO: qt search
// // //             // {
// // //             //     for mt in self.universe.bodies.iter() {
// // //             //         mass_thingies.push(mt)
// // //             //     }
// // //             // }    

// // //             let mut force = Vector2D::new(0., 0.);
// // //             for thingy in bodies2.iter() {
// // //                 force = force + apply_interaction(&body, &thingy);
// // //             }

// // //             let dt = 1.;
// // //             body.velocity = body.velocity + force / body.mass * dt;
// // //             // alert(&format!("{}", body.velocity.x));
// // //             body.location = body.location + body.velocity * dt;
// // //             // alert(&format!("{}", body.location.x));

// // //             // body.update_vel(force);
// // //             // body.update_loc();
// // //             // alert(&format!("{} {}", force.x, force.y));
// // //         }
// // //     }

// // //     pub fn draw(self) {

// // //         // let window = dom_window();
// // //         // let document = dom_document();
// // //         // let body = dom_body();
// // //         let cnv = canvas("canvas_simulation");
// // //         let ctx = ctx(&cnv);

// // //         let canvas_width = f64::from(cnv.width());
// // //         let canvas_height = f64::from(cnv.height());
// // //         // let window_width = window.inner_width().unwrap().as_f64().unwrap();
// // //         // let window_height = window.inner_height().unwrap().as_f64().unwrap();

// // //         // ctx.clear_rect(0., 0., canvas_width, canvas_height);

// // //         const PI: f64 = std::f64::consts::PI;
// // //         const TAU: f64 = 2.0 * PI;

// // //         let x0 = canvas_width / 2.0;
// // //         let y0 = canvas_height / 2.0;
// // //         ctx.set_stroke_style(&JsValue::from_str("purple"));
// // //         ctx.set_fill_style(&JsValue::from_str("purple"));
// // //         let r = 3.;

// // //         for body in &self.universe.bodies {

// // //             let x = body.location.x + x0;
// // //             let y = body.location.y + y0;

// // //             ctx.begin_path();
// // //             ctx.arc(x, y, r, 0.0, TAU).unwrap();
// // //             ctx.stroke();
// // //             ctx.fill();
// // //         }

// // //         // ctx.set_fill_style(&JsValue::from_str("blue"));
// // //         // ctx.begin_path();
// // //         // ctx.arc(f64::from(self.step_idx), f64::from(self.step_idx), r, 0.0, TAU).unwrap();
// // //         // ctx.stroke();
// // //         // ctx.fill();


// // //         // let R = 100.0;
// // //         // let omega = TAU/2.0/60.;
// // //         // let r = R/10.0;

// // //         // let t = f64::from(self.step_idx);
// // //         // let phi = omega * t;

// // //         // let x0 = canvas_width / 2.0;
// // //         // let y0 = canvas_height / 2.0;

// // //         // let x = R * phi.cos() + x0;
// // //         // let y = R * phi.sin() + y0;

// // //         // let p = R/3. * (2.0*omega * t).cos() + x;
// // //         // let q = R/3. * (2.0*omega * t).sin() + y;
// // //         // // let p = x;
// // //         // // let q = y;

// // //         // // if phi/TAU == 0. {
// // //         // ctx.set_stroke_style(&JsValue::from_str("purple"));
// // //         // ctx.set_fill_style(&JsValue::from_str("purple"));
// // //         // // } else if phi/TAU == 1. {
// // //         // //     ctx.set_stroke_style(&JsValue::from_str("black"));
// // //         // //     ctx.set_fill_style(&JsValue::from_str("black"));
// // //         // // }

// // //         // ctx.begin_path();
// // //         // ctx.arc(x0, y0, r, 0.0, TAU);
// // //         // // ctx.move_to(x0, y0);
// // //         // // ctx.line_to(x, y);
// // //         // ctx.stroke();
// // //         // ctx.fill();

// // //         // ctx.begin_path();
// // //         // ctx.arc(x, y, r, 0.0, TAU);
// // //         // // ctx.move_to(x0, y0);
// // //         // // ctx.line_to(x, y);
// // //         // ctx.stroke();
// // //         // ctx.fill();

// // //         // ctx.begin_path();
// // //         // ctx.arc(p, q, r, 0.0, TAU);
// // //         // // ctx.move_to(x0, y0);
// // //         // // ctx.line_to(x, y);
// // //         // ctx.stroke();
// // //         // ctx.fill();
// // //     }
// // // }


// // // pub fn apply_interaction(b1: &Body, b2: &Body) -> Vector2D {

// // //     if b1.id == b2.id {
// // //         return Vector2D::new(0., 0.);
// // //     }

// // //     let G = 1.;

// // //     let foo = b2.location - b1.location;
// // //     let r = (foo.x.powf(2.) + foo.y.powf(2.)).sqrt();

// // //     let F = G * (b1.mass * b2.mass) / r.powf(2.);
// // //     let Fx = F * foo.x / r;
// // //     let Fy = F * foo.y / r;

// // //     // alert(&format!("{} {}", Fx, Fy));

// // //     Vector2D::new(Fx, Fy)
// // // }

