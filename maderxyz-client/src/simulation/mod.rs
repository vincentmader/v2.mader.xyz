use wasm_bindgen::prelude::*;

// use crate::maderxyz-numerics;
// use maderxyz_numerics::state::State;
// use extern_crate::maderxyz_numerics;
// use super::maderxyz_numerics;
// use maderxyz_numerics::State;
// use maderxyz_numerics::state;
// use crate::maderxyz_numerics;


mod engine;
mod renderer;
use engine::Engine;
use renderer::Renderer;
use crate::utils;


// ===================================================

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
        utils::dom::set_panic_hook(); // TODO: helpful?
        self.engine.init();
        self.renderer.init();
    }
    pub fn step(&mut self) {
        self.engine.step();
        self.renderer.display(&self.engine.state);
    }
}
