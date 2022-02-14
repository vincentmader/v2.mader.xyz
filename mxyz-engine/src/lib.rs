
// use std::time;

pub mod boundary;
pub mod config;
pub mod integrator;
pub mod interaction;
pub mod partitioner;
pub mod state;
use integrator::object::variant::ObjIntegratorVariant;
use boundary::object::variant::ObjBoundaryVariant;
use integrator::field::variant::FieldIntegratorVariant;

use crate::state::State;



pub struct Engine {
    sim_id: String,
    pub states: Vec<State>,
    pub iter_idx: usize,
    pub config: config::EngineConfig,
}

impl Engine {

    pub fn new( sim_id: &str ) -> Self {
        Engine { 
            sim_id: String::from(sim_id),
            states: Vec::new(),
            config: config::EngineConfig::new(),
            iter_idx: 0,
        }
    }

    pub fn init( &mut self ) { 
        self.states = Vec::from([
            State::new( &self.sim_id, &mut self.config )
        ]);
    }

    pub fn reset( &mut self ) { 
        self.init();
        self.iter_idx = 0;
    }

    pub fn step( &mut self ) {
        let mut next_state = self.states[self.iter_idx].clone();

        for field in next_state.fields.iter_mut() {

            let stepper = match self.config.fields[field.id].integrator {
                FieldIntegratorVariant::Entire => crate::integrator::field::entire::step,
                FieldIntegratorVariant::RandomBatch => crate::integrator::field::random_batch::step,
            };
            stepper(self.iter_idx, field, &self.states, &self.config);

            // TODO bounds?
        }

        for family in next_state.obj_families.iter_mut() {

            let stepper = match &self.config.obj_families[family.id].integrator {
                ObjIntegratorVariant::EulerExplicit => integrator::object::euler::explicit::step,
                // ObjIntegratorVariant::EulerImplicit => integrator::object::euler::implicit::step,
                // ObjIntegratorVariant::RungeKutta2 => integrator::object::runge_kutta::order_2::step,
                // ObjIntegratorVariant::RungeKutta4 => integrator::object::runge_kutta::order_4::step,
                // ObjIntegratorVariant::Verlet => integrator::object::verlet::step,
                // ObjIntegratorVariant::LeapFrog => integrator::object::leapfrog::step,
            };
            stepper( self.iter_idx, family, &self.states, &self.config );

            let applier = match &self.config.obj_families[family.id].boundary {
                ObjBoundaryVariant::None => boundary::object::none::apply,
                ObjBoundaryVariant::Periodic => boundary::object::periodic::apply,
                ObjBoundaryVariant::WallCollisionElastic => boundary::object::collision::wall::elastic::apply,
                ObjBoundaryVariant::WallCollisionInelastic => boundary::object::collision::wall::inelastic::apply,
            };
            applier(family, &mut self.config.obj_families[family.id]);
        }

        self.states.push(next_state);
        self.iter_idx += 1;
    }
}

