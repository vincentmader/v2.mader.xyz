use wasm_bindgen::prelude::*;

mod engine;
use engine::Engine;
mod renderer;
use renderer::Renderer;
pub mod state;
pub use state::State;


// #[wasm_bindgen]
pub struct Simulation {

    engine: Engine,
    renderer: Renderer,
    page_id: String,

}

// #[wasm_bindgen]
impl Simulation {
    pub fn new(page_id: &str) -> Self {
        let engine = Engine::new(&page_id);
        let renderer = Renderer::new(&page_id);
        let page_id = String::from(page_id);
        Simulation { engine, renderer, page_id }
    }

    pub fn init(&mut self) {
        self.engine.init();
        self.renderer.init();
    }

    pub fn step(&mut self) {
        self.engine.step();
        self.renderer.display(&self.engine.states);
    }
}


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
