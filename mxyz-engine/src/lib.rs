
pub mod boundary;
pub mod config;
pub mod integrator;
pub mod interaction;
pub mod partitioner;
pub mod state;
pub mod system;


pub struct Engine {
        sim_id:         String,
    pub config:         config::EngineConfig,
    pub states:         Vec<state::State>,
    pub systems:        Vec<system::System>,
}

impl Engine {
    // create new Engine struct instance
    pub fn new(sim_id: &str) -> Self {
        Engine { 
            sim_id:     String::from(sim_id),
            config:     config::EngineConfig::new(sim_id),
            states:     Vec::new(),
            systems:    Vec::new(),
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
        for system in self.systems.iter_mut() {
            match system {
                crate::system::System::Field(field) => {
                    use crate::integrator::field::variant::FieldIntegratorVariant;

                    // let state_vector    = &mut field.state_vector;
                    let config          = &field.config;

                    // for i in match state_vector {
                    //     crate::system::StateVector::Bool(e) => {e},
                    //     crate::system::StateVector::Float(e) => {e},
                    // } {}

                    // let old_state       = state_vector[0].clone();

                    for integrator in &config.integrators {
                        match integrator {
                            FieldIntegratorVariant::CellAuto => {
                                // for interaction
                            },
                            FieldIntegratorVariant::FromObjects => {},
                        }
                    }
                }, crate::system::System::ObjectFamily(obj_fam) => {
                    use crate::integrator::object::variant::ObjIntegratorVariant;

                    // let state_vector    = &mut obj_fam.state_vector;
                    let config          = &obj_fam.config;

                    for integrator in &config.integrators {
                        match integrator {
                            ObjIntegratorVariant::EulerExplicit => {

                            },
                            // ObjIntegratorVariant::EulerImplicit => {},
                            // ObjIntegratorVariant::RungeKutta2   => {},
                            // ObjIntegratorVariant::RungeKutta4   => {},
                            // ObjIntegratorVariant::LeapFrog      => {},
                            // ObjIntegratorVariant::Verlet        => {},
                        }
                    }
                }, 
            }
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

