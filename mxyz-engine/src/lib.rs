
// use std::time;

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
    sim_id: String,
    pub engine_setup: EngineSetup, // integrators & bounds for field/obj
    pub states: Vec<State>,
    pub iteration_idx: usize,
    pub config: config::EngineConfig,
    // pub time_of_start: time::Instant,
}

impl Engine {

    pub fn new(
        sim_id: &str, 
        // config: &config::EngineConfig
    ) -> Self {
        Engine { 
            sim_id:          String::from(sim_id),
            states:          Vec::new(),
            // states:          Vec::from([State::new(&self.sim_id, )]),
            config:          config::EngineConfig::new(),
            iteration_idx:   0,
            engine_setup:    EngineSetup::new(),
            // time_of_start:   time::Instant::now(),
        }
    }

    pub fn init(
        &mut self, 
        // config: &config::EngineConfig
    ) { 
        self.states = Vec::from([
            State::new(
                &self.sim_id, 
                // &mut self.engine_setup,
                &mut self.config,
            )
        ]);
    }

    pub fn reset(&mut self) { 
        self.init();
        self.iteration_idx = 0;
    }

    pub fn step(&mut self) {
        let mut state_new = self.states[self.iteration_idx].clone();

        let fields = &mut state_new.fields;
        for mut field in fields.iter_mut() {
            // let integrator = &mut self.engine_setup.field_integrators[field.id];
            // mxyz_utils::dom::console::log(&format!("{}", field.id));
            // let integrator = &mut self.config.fields[field.id];
            // integrator.step(self.iteration_idx, field, &self.states);
            // TODO bounds?
        }

        let families = &mut state_new.object_families;
        for mut family in families.iter_mut() {

            mxyz_utils::dom::console::log(&format!("{}", family.id));
            let integrators = &self.config.obj_families[family.id].integrators;
            for integrator in integrators.iter() {
                integrator.step(self.iteration_idx, &mut family, &self.states, &self.config);
            }

            let boundaries = &mut self.config.obj_families[family.id].boundaries;
            for boundary in boundaries.iter_mut() {
                boundary.apply(&mut family);
            }

            // let integrator = &mut self.engine_setup.object_integrators[family.id];
            // integrator.step(self.iteration_idx, &mut family, &self.states);
            // let boundary = &mut self.engine_setup.object_boundaries[family.id];

        }

        self.states.push(state_new);
        self.iteration_idx += 1;
    }
}

