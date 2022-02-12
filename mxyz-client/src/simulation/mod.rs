
// use std::collections::HashMap;

use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;

mod config;
mod renderer;
use config::Config;
use renderer::Renderer;
mod event_handling;


#[wasm_bindgen]
pub struct Simulation {
    config: Config,
    engine: mxyz_engine::Engine,
    renderer: Renderer,
}

#[wasm_bindgen]
impl Simulation {

    pub fn new(sim_id: &str) -> Self {
        Simulation { 
            config: Config::new(&sim_id),
            engine: mxyz_engine::Engine::new(&sim_id),
            renderer: Renderer::new(&sim_id),
        }
    }

    pub fn init(&mut self) {
        self.engine.init();
        self.renderer.init(&self.engine);  
    }

    pub fn step(&mut self) {  // TODO: multi-thread & async
        if !self.engine.config.is_paused {
            for _ in 0..self.config.nr_of_steps_per_render { 
                self.engine.step(); 
            }
        }
    }

    pub fn render(&mut self) {
        let iter_idx = self.engine.states.len();
        let out_of_bounds = self.renderer.frame_idx >= iter_idx;
        let nr_of_steps_per_render = self.config.nr_of_steps_per_render;
        if !self.renderer.config.is_paused && !out_of_bounds {

            let z = match self.renderer.config.is_iterating_forward { true => 1, false => -1 };
            self.renderer.frame_idx = i32::max(
                0, self.renderer.frame_idx as i32 + z * nr_of_steps_per_render as i32
            ) as usize; 

            self.renderer.display(&self.engine);
        }
    }
}

