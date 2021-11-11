use wasm_bindgen::prelude::*;

use crate::utils::dom::console_log;
// mod cellular_automaton;
// mod n_body_interaction;

// ===================================================

use rand::{Rng};

pub fn get_initial_state_nbody(page_id: &str) -> Vec<f64> {
    let mut rng = rand::thread_rng();

    const TAU: f64 = 2. * 3.14159;

    let mut state: Vec<f64> = Vec::new();
    // TODO: append

    let N = 300; // TODO
    for id in 0..N {
        let mut phi: f64 = rng.gen();
        phi *= TAU;
        let phi = id as f64 / N as f64 * TAU;

        let mut r: f64 = rng.gen();
        // r *= 0.34;
        // r += 0.12;
        // let r = 0.5;
        let r = 0.12 + 0.34 * id as f64 / N as f64;;

        let m = 0.0001;
        let x = r * phi.cos();
        let y = r * phi.sin();
        let u = 0.;
        let v = 0.;
        state.extend_from_slice(&[m,x,y,u,v]);
    }
    state
}

pub fn get_initial_state_cellauto(page_id: &str) -> Vec<f64> {
    let mut rng = rand::thread_rng();

    // TODO: append
    let foo: u32 = match page_id {
        "ising" => 150,
        _ => 16,  // TODO: ?
    };
    let nr_of_cells = foo.pow(2);

    let mut state: Vec<f64> = Vec::new();
    for _ in 0..nr_of_cells {
        let spin_up: bool = rng.gen();
        if spin_up {
            state.push(1.);
        } else {
            state.push(-1.);
        }
    }
    state
}

pub fn step_nbody(page_id: &str, state: &Vec<f64>) -> Vec<f64> {

    let n = 5;
    let N = state.len() / n;

    let G: f64 = 0.01;
    let eps: f64 = 0.05;
    let dt: f64 = 0.1;

    let mut state_clone = state.clone();

    for body_id in 0..N {
        let x1 = state[n*body_id+1];
        let y1 = state[n*body_id+2];
            // let m1 = state[n*body_id];
            // let u1 = state[n*body_id+3];
            // let v1 = state[n*body_id+4];

        let mut acc_x = 0.;
        let mut acc_y = 0.;
        for body_jd in 0..N {
            // no self-interaction
            if body_id == body_jd {
                continue
            }
            // read from state vector
            let x2 = state[n*body_jd+1];
            let y2 = state[n*body_jd+2];
            let m2 = state[n*body_jd];
                // let u2 = state[n*body_jd+3];
                // let v2 = state[n*body_jd+4];
            // get relative placement
            let delta_x = x2 - x1;
            let delta_y = y2 - y1;
            let dist = (delta_x.powf(2.)+delta_y.powf(2.)).sqrt();
            let unit_x = delta_x / dist;
            let unit_y = delta_y / dist;
            // euler
            acc_x += G*m2 / (dist.powf(2.) + eps.powf(2.)) * unit_x;
            acc_y += G*m2 / (dist.powf(2.) + eps.powf(2.)) * unit_y;
        }
        state_clone[n*body_id + 3] += acc_x * dt;
        state_clone[n*body_id + 4] += acc_y * dt;    
    }

    // let mut state_clonee = state_clone.clone();
    for body_id in 0..N {
        let u = state_clone[n*body_id + 3];
        let v = state_clone[n*body_id + 4];
        state_clone[n*body_id + 1] += u * dt;
        state_clone[n*body_id + 2] += v * dt;
        // if body_id == 0 {
        // console_log(&format!("{}", u));
        // }
    }

    state_clone
}

pub fn step_cellauto(page_id: &str, state: &Vec<f64>) -> Vec<f64>{
    const M: usize = 200;  // TODO: make changeable

    let mut rng = rand::thread_rng();

    let mut state_clone = state.clone();
    let N = (state.len() as f64).sqrt() as usize; // TODO: only works for square

    let mut cell_ids: Vec<usize> = vec![];
    for _ in 0..M {
        cell_ids.push(rng.gen_range(0..N.pow(2)));
    }

    let k = 1.;
    let mu = 1.;
    let J = 1.;
    let B = 0.;  // TODO: make changeable
    let T = 1.;  // TODO: make changeable
    let beta = k * T;

    for cell_id in cell_ids.iter() {

        let spin = state[*cell_id];
        let y = cell_id / N;
        let x = cell_id % N;

        let mut dE = 0.;
        let arr: [i32; 3] = [-1, 0, 1];
        for dy in arr.iter() {
            for dx in arr.iter() {
                // prevent self-self interaction
                if (*dx == 0) && (*dy == 0) {
                   continue
                }
                // apply periodic boundaries
                let mut a = x as i32 + dx;  // new coord
                let mut b = y as i32 + dy;
                let n = N as i32;
                if a < 0 {a += n} else if a >= n {a -= n}
                if b < 0 {b += n} else if b >= n {b -= n}
                let a = a as usize;  // convert back
                let b = b as usize;
                // get neighboring spin value
                let other = &state[b*N+a];
                // apply energy change
                dE -= -J * spin * other;
                dE += -J * -spin * other;
           }
        }
        // change in energy through flip
        dE -= B*mu*spin;
        dE += B*mu*-spin;
        // probability for flip
        let p = (-dE * beta).exp();
        let rand: f64 = rng.gen();
        if rand < p {
            state_clone[y*N+x] *= -1.;
        }
    }
    state_clone
}

pub struct Engine {
    category: String,
    page_id: String,
    state: Vec<f64>,
}
impl Engine {
    pub fn new(category: &str, page_id: &str) -> Self {
        let state: Vec<f64> = Vec::new();
        Engine {
            category: String::from(category), 
            page_id: String::from(page_id), 
            state,
        }
    }
    pub fn init(&mut self) {
        // TODO: initialize state vector
        self.state = match self.page_id.as_str() {
            "3body-fig8" | "3body-moon" 
                => get_initial_state_nbody(&self.page_id),
            "ising" 
                => get_initial_state_cellauto(&self.page_id), 
            _ => Vec::new() // TODO: crash?
        }
    }
    pub fn step(&mut self) {
        // TODO: choose forward function, apply
        self.state = match self.page_id.as_str() {
            "3body-fig8" | "3body-moon" 
                => step_nbody(&self.page_id, &self.state),
            "ising" 
                => step_cellauto(&self.page_id, &self.state),
            _ => Vec::new() // TODO: ?
        };
    }
}


use crate::utils::dom::Canvas;

fn display_nbody(page_id: &str, state: &Vec<f64>) {
    let mut canvas = Canvas::new("canvas_main", true); // TODO: page_id

    // drawing setup
    let color = "white";
    canvas.set_fill_style(&color);
    canvas.set_stroke_style(&color);
    // clear canvas
    canvas.clear();

    let n = 5;
    let N = state.len() / n;
    for id in 0..N {
        let m = state[n*id];
        let x = state[n*id + 1];
        let y = state[n*id + 2];
            // let u = state[n*id + 3];
            // let v = state[n*id + 4];

        let r = 0.01;

        canvas.draw_circle((x,y), r, true);
    }
}
fn display_cellauto(page_id: &str, state: &Vec<f64>) {
    let mut canvas = Canvas::new("canvas_main", false); // TODO: page_id
    // clear canvas
    canvas.clear();

    // draw on canvas
    // match page_id {
    //     "ising" => ising_draw(state),
    //     _ => ()
    // }

    let N = (state.len() as f64).sqrt() as usize; // TODO: only works for square

    canvas.set_fill_style("white");
    for i in 0..N {
        for j in 0..N {
            let N = N as f64;
            let x = i as f64 / N;
            let y = j as f64 / N;
            let z = 0.8; // TODO move elsewhere
            let center = (
                x + (1.-z) / (2.*N), 
                y + (1.-z) / (2.*N)
            );
            let width = 1. / N * z;
            let height = 1. / N * z;
            if state[i*N as usize+j] == 1. {
                canvas.fill_rect(center, width, height);
                // console_log(&format!("{}, {}", i, j))
            }
        }
    }
}


// pub fn ising_draw(state: &Vec<f64>) {}

// fn display_graph(page_id: u32, state: &Vec<f64>) {
//     let mut canvas = Canvas::new("canvas_main", false); // TODO: page_id
//     canvas.fill_rect((10., 10.), 20.,30.);
// }

pub struct Renderer {
    category: String,
    page_id: String,
}
impl Renderer {
    pub fn new(category: &str, page_id: &str) -> Self {
        Renderer {
            category: String::from(category), 
            page_id: String::from(page_id), 
        }
    }
    pub fn init(&mut self) {
        // TODO: setup event listeners, charts, ...
        match self.page_id {
            // 0 => (),
            _ => ()
        }
    }
    pub fn display(&mut self, state: &Vec<f64>) {
        // TODO: 
        match self.page_id.as_str() {
            "3body-fig8" | "3body-moon" 
                => display_nbody(&self.page_id, state),
            "ising" 
                => display_cellauto(&self.page_id, state),
            // "mc-pi" => display_graph(&self.page_id, state),
            _ => ()
        }
    }
}

#[wasm_bindgen]
pub struct Simulation {
    engine: Engine,
    renderer: Renderer,
}
#[wasm_bindgen]
impl Simulation {
    pub fn new(category: &str, page_id: &str) -> Self {
        let engine = Engine::new(category, page_id);
        let renderer = Renderer::new(category, page_id);
        Simulation { engine, renderer }
    }
    pub fn init(&mut self) {
        self.engine.init();
        self.renderer.init();
    }
    pub fn step(&mut self) {
        self.engine.step();
        self.renderer.display(&self.engine.state);
    }
}


// SIMULATION
// ===================================================
//mod body;
// mod universe;
// mod engine;
// mod renderer;
// mod renderer2;
// pub use body::Body;
// pub use universe::Universe;
// pub use engine::Engine;
// pub use renderer::Renderer;
// pub use renderer2::Renderer2;


//#[wasm_bindgen]
//pub struct Simulation {
//    page_id: String,
//    engine: Engine,
//    renderer: Renderer,
//    step_idx: u32,
//}
//#[wasm_bindgen]
//impl Simulation {
//    pub fn new(page_id: String) -> Simulation {

//        let step_idx: u32 = 0;
//        let nr_of_bodies: u32 = 100;  // TODO: make changeable
//        let dt = 0.01;                // TODO: move elsewhere

//        let engine = Engine::new(nr_of_bodies, dt);

//        let renderer = match page_id.as_str() {
//            "3body_fig8" => Renderer::new(),
//            // "3body_moon" => Renderer2::new(),
//            _ => Renderer::new() // default
//        };

//        Simulation {
//            page_id, engine, renderer, step_idx, 
//        }
//    }
//    pub fn init(&mut self) {
//        self.engine.init();
//        self.renderer.init();
//    }
//    pub fn step(&mut self) {
//        self.engine.step();
//        self.renderer.display(&self.engine.universe);
//        self.step_idx += 1;
//    }
//}































//////
//////
////pub struct EngineNBody {

////}
////pub struct RendererNBody {

////}

////pub enum EngineType {
////    NBody // (EngineNBody), // gravity, collisions, boids, charge+-, friction
////    // CellularAutomaton(EngineCellularAutomaton), // ising, heat, gol, patterns
////    // Pendulum(EnginePendulum), // single, double, chaos
////}

////pub enum RendererType {
////    // Plot1D(RendererPlot1D),
////    // Plot2D(RendererPlot2D), // pendulum, lissajous, pi
////    // CellularAutomaton(RendererCellularAutomaton),
////    NBody // (RendererNBody),
////}


////mod NBody {
////    // N-body engine
////    pub struct Engine {
////        step_idx: u32, 
////    }
////    impl Engine {
////        pub fn new() -> Self {
////            let step_idx: u32 = 0;
////            Engine {
////                step_idx,
////            }
////        }
////        pub fn init(&mut self) {}
////        pub fn update(&mut self) {}
////        pub fn step(self) {}
////    }

////    // N-body renderer
////    pub struct Renderer {
////        frame_idx: u32,
////    }
////    impl Renderer {
////        pub fn new() -> Self {
////            let frame_idx: u32 = 0;
////            Renderer {
////                frame_idx,
////            }
////        }
////        pub fn init(&mut self) {}
////        pub fn update(&mut self) {}
////        pub fn draw(self) {}
////    }
////}

////pub trait Animation {
////    fn new() -> Self;
////    fn init(&self) {}
////    fn update(&self) {}
////}

////#[wasm_bindgen]
////pub struct NB {
////    animation_id: String,
////    engine: NBody::Engine,
////    renderer: NBody::Renderer,
////}
////#[wasm_bindgen]
////impl NB {
////    pub fn new(animation_id: String) -> Self {

////        let engine = match animation_id.as_str() {
////            "nbody_moon" => NBody::Engine::new(),
////            _ => NBody::Engine::new(),
////        //         EngineType::NBody,
////        //     _ => EngineType::NBody, // default
////        };
////        let renderer = match animation_id.as_str() {
////            "nbody_moon" => NBody::Renderer::new(),
////            _ => NBody::Renderer::new(),
////        //     "nbody_moon" => RendererType::NBody,
////        //     _ => RendererType::NBody, // default
////        };

////        NB {
////            animation_id,
////            engine,
////            renderer
////        }
////    }
////    pub fn init(&mut self) {
////        // self.engine.init();
////        // self.renderer.init();
////    }
////    pub fn update(&mut self) {
////        // self.engine.step();
////        // self.renderer.display(&self.engine.universe);

////    }
////    pub fn render(&self) {}
////}


//////         let nr_of_bodies: u32 = 100;  // TODO: make changeable
//////         let dt = 0.01;                // TODO: move elsewhere

//////         let engine = Engine::new(nr_of_bodies, dt);

//////         let renderer = match page_id.as_str() {
//////             "3body_fig8" => Renderer::new(),
//////             // "3body_moon" => Renderer2::new(),
//////             _ => Renderer::new() // default
//////         };

//////         Simulation {
//////             page_id, engine, renderer, step_idx, 
//////         }
//////     }
//////     pub fn init(&mut self) {
//////         self.engine.init();
//////         self.renderer.init();
//////     }
//////     pub fn step(&mut self) {
//////         self.engine.step();
//////         self.renderer.display(&self.engine.universe);
//////         self.step_idx += 1;
//////     }
////// }


////// pub struct Animation<E: Engine2, R: Renderer2> {
//////     // renderer: &<'a> T,
//////     engine: E,
//////     renderer: R,
////// }

////// pub trait Animation {
//////     fn new() -> Self;
//////     fn init(&self) {}
//////     fn render(&self) {}
////// }

////// pub trait Renderer2 {
//////     fn new() -> Self;
//////     fn display(&self) {}
////// }

////// pub trait Engine2 {
//////     fn new() -> Self;
//////     fn step(&self) {}
////// }

////// // N-Body

////// pub struct NBodyGravity {
//////     // universe: Universe, (state?)
////// }
////// // impl Animation for NBodyGravity {
////// impl NBodyGravity {
//////     fn new() -> Self {
//////         NBodyGravity {}
//////     }
//////     fn init(&self) {}
//////     fn render(&self) {}
////// }

////// // Ising 

////// pub struct IsingModel {
//////     // state: 
////// }
////// // impl Animation for IsingModel {
////// impl IsingModel {
//////     fn new() -> Self {
//////         IsingModel {}
//////     }
//////     fn init(&self) {}
//////     fn render(&self) {}
////// }


