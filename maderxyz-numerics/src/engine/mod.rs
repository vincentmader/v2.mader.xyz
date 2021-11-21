
use crate::state::State;


pub struct Engine {

    pub states: Vec<State>,
    page_id: String,

}
impl Engine {

    pub fn new(page_id: &str) -> Self {
        let states: Vec<State> = Vec::new();
        let page_id = String::from(page_id);
        Engine { states, page_id }
    }

    pub fn init(&mut self) {
        let initial_state = State::new(&self.page_id);
        self.states.push(initial_state);
    }

    pub fn step(&mut self) {

        match self.page_id.as_str() {
            "3body_fig8" | "3body_moon" | 
            "nbody_flowers" | "nbody_asteroids" => {
                // update_bodies(inte)
            },
            "charge-interaction" => {},
            "ising" => {},
            _ => {}
        }
    }
}
