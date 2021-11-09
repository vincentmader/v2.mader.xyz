use wasm_bindgen::prelude::*;

mod engine;
use engine::Engine;
mod renderer;
use renderer::Renderer;

#[wasm_bindgen]
pub struct CellularAutomaton {
    sim_id: String,
    engine: Engine,
    renderer: Renderer,
    N: usize,
}
#[wasm_bindgen]
impl CellularAutomaton {
    pub fn new(sim_id: String) -> Self {

        // use std::collections::HashMap;
        // let params = HashMap::from([
        //     ("sim_id", sim_id.clone()),
        //     ("N", 100),
        // ]);
        let N: usize = 150; // TODO: move where?
        let engine = Engine::new(sim_id.clone(), N);
        let renderer = Renderer::new(sim_id.clone());
        CellularAutomaton {
            sim_id, engine, renderer, N
        }
    }
    pub fn init(&mut self) {
        self.engine.init(self.N);
        self.renderer.init();
    }
    pub fn step(&mut self) {
        self.engine.step(self.N);
        self.renderer.draw(self.N, &self.engine.state);
    }
}
