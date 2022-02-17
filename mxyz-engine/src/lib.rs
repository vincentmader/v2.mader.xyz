
pub mod boundary;
pub mod config;
pub mod integrator;
pub mod interaction;
pub mod partitioner;
pub mod state;


pub struct Engine {
        sim_id:         String,
    pub config:         config::EngineConfig,
    pub states:         Vec<state::State>,
}

impl Engine {
    // create new Engine struct instance
    pub fn new(sim_id: &str) -> Self {
        Engine { 
            sim_id:     String::from(sim_id),
            config:     config::EngineConfig::new(),
            states:     Vec::new(),
        }
    }
    // initialize state vector
    pub fn init(&mut self) { 
        self.states = Vec::from([state::State::new(&self.sim_id, &mut self.config)]);
    }
    // get state-vector for next time-step
    pub fn step(&mut self) {
        // clone state-vector
        let mut next_state = self.states[self.config.iter_idx].clone();
        // step fields & obj-families
        for mut field in next_state.fields.iter_mut() {
            integrator::field::step(&mut field, &self.states, &self.config);
        }
        for mut family in next_state.obj_families.iter_mut() {
            integrator::object::step(&mut family, &self.states, &self.config);
        }
        // update state-vector & increment iteration-index
        self.states.push(next_state);
        self.config.iter_idx += 1;
    }
    // reset Engine struct instance
    pub fn reset(&mut self) { 
        self.init();
        self.config.iter_idx = 0;
    }
}

