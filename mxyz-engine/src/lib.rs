
pub mod boundary;
pub mod integrator;
pub mod interaction;
pub mod partitioner;
pub mod state;

use crate::state::State;
mod setup;
use setup::EngineSetup;

pub mod config;


pub struct Engine {

    sim_id: String,  // simulation id
    pub engine_setup: EngineSetup, // integrators & bounds for field/obj
    pub states: Vec<State>,
    pub iteration_idx: usize,

}
impl Engine {

    pub fn new(sim_id: &str) -> Self {
        Engine { 
            sim_id:          String::from(sim_id),
            engine_setup:    EngineSetup::new(),
            states:          Vec::new(),
            iteration_idx:   0,
        }
    }

    pub fn init(&mut self) { 
        // initialize states
        self.states.push(
        // self.states = 
        //     Vec::from([
            State::new(&self.sim_id, &mut self.engine_setup)
        // ]);
        )
    }

    pub fn reset(&mut self) { 

        self.iteration_idx = 0;
        self.init();  // initializes state vector

    }

    pub fn step(&mut self) {

        let mut state_new = self.states[self.iteration_idx].clone();
        let fields = &mut state_new.fields;
        for mut field in fields.iter_mut() {
            let field_id = field.id;
            let integrator = &mut self.engine_setup.field_integrators[field_id];
            integrator.step(self.iteration_idx, field, &self.states);
            // TODO bounds?

        }

        let families = &mut state_new.object_families;
        for mut family in families.iter_mut() {
            let family_id = family.id;
            let integrator = &mut self.engine_setup.object_integrators[family_id];
            integrator.step(self.iteration_idx, &mut family, &self.states);
            let boundary = &mut self.engine_setup.object_boundaries[family_id];
            boundary.apply(&mut family);

        }

        self.states.push(state_new);
        self.iteration_idx += 1;
    }
}

