use wasm_bindgen::prelude::*;

mod engine;
use engine::Engine;
mod renderer;
use renderer::Renderer;

#[wasm_bindgen]
pub struct CellularAutomaton {
    page_id: String,
    engine: Engine,
    renderer: Renderer,
    N: usize,
}
#[wasm_bindgen]
impl CellularAutomaton {
    pub fn new(page_id: String) -> Self {

        // set grid geometry 
        let N: usize = match page_id.as_str() {
            "ising" => 150,
            "diffusion" => 50,
            _ => 4,
        };
        // create parameter hashmap
        // use std::collections::HashMap;
        // let params: HashMap<&str, String> = HashMap::from([
        //     ("page_id", page_id.clone()),
        //     ("N", format!("{}", N)),
        // ]);

        let engine = Engine::new(
            page_id.clone(), 
            N
        ); 
        let renderer = Renderer::new(
            page_id.clone()
        );
        CellularAutomaton {
            page_id, engine, renderer, N
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
