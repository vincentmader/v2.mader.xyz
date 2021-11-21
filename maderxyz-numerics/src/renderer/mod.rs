
use crate::state::State;


pub struct Renderer {

    page_id: String,

}
impl Renderer {

    pub fn new(page_id: &str) -> Self {
        Renderer {
            page_id: String::from(page_id),
        }
    }

    pub fn init(&mut self) {

    }

    pub fn display(&self, states: &Vec<State>) {

    }
}
