
pub mod simulation;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct Client {

}
#[wasm_bindgen]
impl Client {

    pub fn new() -> Self {
        Client {

        }
    }

    pub fn init(&mut self) {
        // utils::dom::set_panic_hook(); // TODO: helpful?
    }

}

