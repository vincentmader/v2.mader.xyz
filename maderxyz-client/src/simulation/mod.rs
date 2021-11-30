
use wasm_bindgen::prelude::*;

mod renderer;
use renderer::Renderer;
use maderxyz_numerics::engine::Engine;
use crate::utils;



#[wasm_bindgen]
pub struct Simulation {
    engine: Engine,
    renderer: Renderer,
}
#[wasm_bindgen]
impl Simulation {
    pub fn new(page_id: &str) -> Self {
        let engine = Engine::new(page_id);
        let renderer = Renderer::new(page_id);
        Simulation { engine, renderer }
    }
    pub fn init(&mut self) {
        utils::dom::set_panic_hook(); // TODO: helpful?
        self.engine.init();
        self.renderer.init();
    }
    pub fn step(&mut self) {  // TODO: multithread & async
        self.renderer.display(&self.engine.states);
        for _ in 0..1{ // TODO
            self.engine.step();
        }
    }
}

