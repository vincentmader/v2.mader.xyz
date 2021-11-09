
// Imports
// ===================================================

use wasm_bindgen::prelude::*;

mod body;
mod universe;
mod engine;
mod renderer;
mod renderer2;
pub use body::Body;
pub use universe::Universe;
pub use engine::Engine;
pub use renderer::Renderer;
// pub use renderer2::Renderer2;

// CELLULAR AUTOMATON
// ===================================================

mod cellular_automaton;


// SIMULATION
// ===================================================

#[wasm_bindgen]
pub struct Simulation {
    sim_id: String,
    engine: Engine,
    renderer: Renderer,
    step_idx: u32,
}
#[wasm_bindgen]
impl Simulation {
    pub fn new(sim_id: String) -> Simulation {

        let step_idx: u32 = 0;
        let nr_of_bodies: u32 = 100;  // TODO: make changeable
        let dt = 0.01;                // TODO: move elsewhere

        let engine = Engine::new(nr_of_bodies, dt);

        let renderer = match sim_id.as_str() {
            "3body_fig8" => Renderer::new(),
            // "3body_moon" => Renderer2::new(),
            _ => Renderer::new() // default
        };

        Simulation {
            sim_id, engine, renderer, step_idx, 
        }
    }
    pub fn init(&mut self) {
        self.engine.init();
        self.renderer.init();
    }
    pub fn step(&mut self) {
        self.engine.step();
        self.renderer.display(&self.engine.universe);
        self.step_idx += 1;
    }
}































//
//
pub struct EngineNBody {

}
pub struct RendererNBody {

}

pub enum EngineType {
    NBody // (EngineNBody), // gravity, collisions, boids, charge+-, friction
    // CellularAutomaton(EngineCellularAutomaton), // ising, heat, gol, patterns
    // Pendulum(EnginePendulum), // single, double, chaos
}

pub enum RendererType {
    // Plot1D(RendererPlot1D),
    // Plot2D(RendererPlot2D), // pendulum, lissajous, pi
    // CellularAutomaton(RendererCellularAutomaton),
    NBody // (RendererNBody),
}


mod NBody {
    // N-body engine
    pub struct Engine {
        step_idx: u32, 
    }
    impl Engine {
        pub fn new() -> Self {
            let step_idx: u32 = 0;
            Engine {
                step_idx,
            }
        }
        pub fn init(&mut self) {}
        pub fn update(&mut self) {}
        pub fn step(self) {}
    }

    // N-body renderer
    pub struct Renderer {
        frame_idx: u32,
    }
    impl Renderer {
        pub fn new() -> Self {
            let frame_idx: u32 = 0;
            Renderer {
                frame_idx,
            }
        }
        pub fn init(&mut self) {}
        pub fn update(&mut self) {}
        pub fn draw(self) {}
    }
}

pub trait Animation {
    fn new() -> Self;
    fn init(&self) {}
    fn update(&self) {}
}

#[wasm_bindgen]
pub struct NB {
    animation_id: String,
    engine: NBody::Engine,
    renderer: NBody::Renderer,
}
#[wasm_bindgen]
impl NB {
    pub fn new(animation_id: String) -> Self {

        let engine = match animation_id.as_str() {
            "nbody_moon" => NBody::Engine::new(),
            _ => NBody::Engine::new(),
        //         EngineType::NBody,
        //     _ => EngineType::NBody, // default
        };
        let renderer = match animation_id.as_str() {
            "nbody_moon" => NBody::Renderer::new(),
            _ => NBody::Renderer::new(),
        //     "nbody_moon" => RendererType::NBody,
        //     _ => RendererType::NBody, // default
        };

        NB {
            animation_id,
            engine,
            renderer
        }
    }
    pub fn init(&mut self) {
        // self.engine.init();
        // self.renderer.init();
    }
    pub fn update(&mut self) {
        // self.engine.step();
        // self.renderer.display(&self.engine.universe);

    }
    pub fn render(&self) {}
}


//         let nr_of_bodies: u32 = 100;  // TODO: make changeable
//         let dt = 0.01;                // TODO: move elsewhere

//         let engine = Engine::new(nr_of_bodies, dt);

//         let renderer = match sim_id.as_str() {
//             "3body_fig8" => Renderer::new(),
//             // "3body_moon" => Renderer2::new(),
//             _ => Renderer::new() // default
//         };

//         Simulation {
//             sim_id, engine, renderer, step_idx, 
//         }
//     }
//     pub fn init(&mut self) {
//         self.engine.init();
//         self.renderer.init();
//     }
//     pub fn step(&mut self) {
//         self.engine.step();
//         self.renderer.display(&self.engine.universe);
//         self.step_idx += 1;
//     }
// }


// pub struct Animation<E: Engine2, R: Renderer2> {
//     // renderer: &<'a> T,
//     engine: E,
//     renderer: R,
// }

// pub trait Animation {
//     fn new() -> Self;
//     fn init(&self) {}
//     fn render(&self) {}
// }

// pub trait Renderer2 {
//     fn new() -> Self;
//     fn display(&self) {}
// }

pub trait Engine2 {
    fn new() -> Self;
    fn step(&self) {}
}

// N-Body

pub struct NBodyGravity {
    // universe: Universe, (state?)
}
// impl Animation for NBodyGravity {
impl NBodyGravity {
    fn new() -> Self {
        NBodyGravity {}
    }
    fn init(&self) {}
    fn render(&self) {}
}

// Ising 

pub struct IsingModel {
    // state: 
}
// impl Animation for IsingModel {
impl IsingModel {
    fn new() -> Self {
        IsingModel {}
    }
    fn init(&self) {}
    fn render(&self) {}
}


