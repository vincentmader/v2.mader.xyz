
use wasm_bindgen::prelude::*;

mod config;
mod renderer;
mod event_handling;


#[wasm_bindgen]
pub struct Simulation {
    config:      config::Config,
    engine: mxyz_engine::Engine,
    renderer:  renderer::Renderer,
}

#[wasm_bindgen]
impl Simulation {

    pub fn new(sim_id: &str) -> Self {
        Simulation { 
            config:       config::Config::new(&sim_id),
            engine:  mxyz_engine::Engine::new(&sim_id),
            renderer: renderer::Renderer::new(&sim_id),
        }
    }

    pub fn init(&mut self) {
        self.engine.init();
        self.renderer.init(&self.engine);  
    }

    pub fn step(&mut self) {  // TODO: multi-thread & async
        if self.engine.config.is_paused { return; }
        for _ in 0..self.config.nr_of_steps_per_render { self.engine.step(); }
    }

    pub fn render(&mut self) {
        let out_of_bounds          = self.renderer.frame_idx >= self.engine.config.iter_idx;
        let nr_of_steps_per_render = self.config.nr_of_steps_per_render as i32;
        let is_paused              = self.renderer.config.is_paused;
        let frame_idx              = self.renderer.frame_idx as i32;

        if !is_paused && !out_of_bounds {
            let z = match self.renderer.config.is_iterating_forward { true => 1, false => -1 };
            self.renderer.frame_idx = i32::max(0, frame_idx+z*nr_of_steps_per_render) as usize; 
            self.renderer.display(&self.engine);
        }
    }
}

