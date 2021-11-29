
// use std::cmp;

use crate::state::State;
use crate::state::ObjectType;
use crate::integrators;
use crate::integrators::Integrator;
use crate::interactions::ObjectInteraction;


pub struct Engine {

    pub states: Vec<State>,
    page_id: String,
    iteration_step: usize,

}
impl Engine {

    pub fn new(page_id: &str) -> Self {
        let states: Vec<State> = Vec::new();
        let page_id = String::from(page_id);

        Engine { states, page_id, iteration_step: 0 }
    }

    pub fn init(&mut self) {
        let initial_state = State::new(&self.page_id);
        self.states.push(initial_state);
    }

    pub fn step(&mut self) {
        let current_state = &self.states[self.iteration_step];
        let mut next_state = current_state.clone();
        next_state.iteration_idx += 1;
        // step objects, fields, cells & spins 
        Self::step_objects(&mut next_state, &current_state);
        // TODO fields
        // TODO cells
        // TODO spins
        self.states.push(next_state);
        self.iteration_step += 1;
    }

    fn step_objects(next_state: &mut State, current_state: &State) {
        for (family_idx, object_family) in next_state.object_families.iter_mut().enumerate() {
            // don't apply forces to statics  // TODO statics on "rails"?
            if matches!(object_family.object_type, ObjectType::Static) { continue }
            // setup integrator
            let integrator = match object_family.integrator {
                Integrator::EulerExplicit => integrators::euler_explicit::step,
                // Integrator::EulerImplicit => integrators::euler_implicit::step,
                // Integrator::RungeKutta2 => integrators::runge_kutta_2::step,
                // Integrator::RungeKutta4 => integrators::runge_kutta_4::step,
                // Integrator::LeapFrog => integrators::leap_frog::step,
                // Integrator::Verlet => integrators::verlet::step,
            };
            // loop over other object families
            for (other_idx, other_family) in current_state.object_families.iter().enumerate() {
                let family_indices = (family_idx, other_idx);
                // don't apply influence of low-mass particles
                if matches!(other_family.object_type, ObjectType::Particle) { continue };
                // choose relevant interactions (both families must "feel" them)  
                let mut interactions: Vec<ObjectInteraction> = Vec::new();
                for object_interaction in object_family.interactions.iter() {
                    for other_interaction in other_family.interactions.iter() {
                        if !matches!(object_interaction, other_interaction) {continue}  // TODO does this work?
                        if !interactions.contains(object_interaction) {
                            interactions.push(object_interaction.clone());
                        }
                    }
                }
                // use integrator to apply interaction -> step object family
                integrator(
                    object_family, other_family, 
                    &interactions, family_indices
                );
            }
        }
    }
}

