
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
        let frame_idx              = self.renderer.config.frame_idx as i32;
        let out_of_bounds          = frame_idx > self.engine.config.iter_idx as i32;
        let nr_of_steps_per_render = self.config.nr_of_steps_per_render as i32;
        let is_paused              = self.renderer.config.is_paused;

        if !is_paused && !out_of_bounds {
            let z = match self.renderer.config.is_iterating_forward { true => 1, false => -1 };
            self.renderer.config.frame_idx = i32::max(0, frame_idx+z*nr_of_steps_per_render) as usize; 
            self.renderer.display(&self.engine);
        // } else {
        //     self.renderer.canvases[0].clear(); // TODO show iter-idx even when paused
        //     self.renderer.display_hud(&self.engine);
        }
    }
}

