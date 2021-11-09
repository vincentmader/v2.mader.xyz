use wasm_bindgen::prelude::*;

mod engine;
use engine::Engine;
mod renderer;
use renderer::Renderer;

#[wasm_bindgen]
pub struct NBodyInteraction {
    page_id: String,
    engine: Engine,
    renderer: Renderer,
}
#[wasm_bindgen]
impl NBodyInteraction {
    pub fn new(page_id: String) -> Self {

        let engine = Engine::new(
            page_id.clone(), 
        ); 
        let renderer = Renderer::new(
            page_id.clone()
        );
        NBodyInteraction {
            page_id, engine, renderer
        }
    }
    pub fn init(&mut self) {
        self.engine.init();
        self.renderer.init();
    }
    pub fn step(&mut self) {
        self.engine.step();
        self.renderer.draw(&self.engine.state);
    }
}
