use wasm_bindgen::prelude::*;

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
