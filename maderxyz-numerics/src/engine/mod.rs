
use std::collections::HashMap;

use crate::integrators;
use crate::integrators::object::Integrator as ObjectIntegrator;
use crate::integrators::field::Integrator as FieldIntegrator;
use crate::state::State;
use crate::state::ObjectType;


pub struct Engine {

    page_id: String,
    pub states: Vec<State>,
    pub iteration_step: usize,
    pub is_paused: bool,

}
impl Engine {

    pub fn new(page_id: &str) -> Self {

        let states: Vec<State> = Vec::new();
        let page_id = String::from(page_id);
        let is_paused = false;

        Engine { states, page_id, iteration_step: 0, is_paused }
    }

    pub fn init(&mut self) {

        let initial_state = State::new(&self.page_id);
        self.states = Vec::from([initial_state]);

    }

    pub fn step(&mut self, parameters: &HashMap<String, String>) {

        if self.is_paused { return (); }

        let current_state = &self.states[self.iteration_step];
        let mut next_state = current_state.clone();
        next_state.iteration_idx += 1;

        Self::step_objects(&mut next_state, &current_state);
        Self::step_fields(&mut next_state, &current_state);  // TODO 1D, 2D, 3D, bool, spinor...

        self.states.push(next_state);
        self.iteration_step += 1;

    }

    fn step_objects(next_state: &mut State, current_state: &State) {

        // loop over object families
        for object_family in next_state.object_families.iter_mut() {
            // don't apply forces to statics  // TODO statics on "rails"?
            if matches!(object_family.object_type, ObjectType::Static) { continue }
            // loop over interactions
            for interaction in object_family.interactions.clone().iter() {  // TODO get rid of clone
                // setup integrator
                let integrator = match interaction.integrator {
                    ObjectIntegrator::EulerExplicit => integrators::object::euler_explicit::step,
                    ObjectIntegrator::EulerImplicit => integrators::object::euler_implicit::step,
                    ObjectIntegrator::RungeKutta2 => integrators::object::runge_kutta_2::step,
                    ObjectIntegrator::RungeKutta4 => integrators::object::runge_kutta_4::step,
                    ObjectIntegrator::LeapFrog => integrators::object::leap_frog::step,
                    ObjectIntegrator::Verlet => integrators::object::verlet::step,
                };
                // loop over object families (including this one)
                for other_family in current_state.object_families.iter() {
                    // don't apply influence of low-mass particles
                    if matches!(other_family.object_type, ObjectType::Particle) { continue };
                    // only apply interactions when both families "feel" them
                    for other_interaction in other_family.interactions.iter() {
                        if interaction.interaction_variant != other_interaction.interaction_variant { continue }
                    }
                    integrator(object_family, other_family, &interaction);
                }
            }
        }
    }

    fn step_fields(next_state: &mut State, current_state: &State) {

        // loop over fields
        for (field_idx, field) in next_state.fields.iter_mut().enumerate() {
            // loop over interactions
            for interaction in current_state.fields[field_idx].interactions.iter() {
                // setup integrator
                let integrator = match interaction.integrator {
                    FieldIntegrator::BatchWise => integrators::field::batch_wise::step,
                    FieldIntegrator::Entire => integrators::field::entire::step,
                };
                // loop over fields (including this one)
                for other_field in current_state.fields.iter() {
                    // only apply interactions when both fields "feel" them
                    for other_interaction in other_field.interactions.iter() {
                        if interaction.interaction_variant != other_interaction.interaction_variant { continue }
                    }
                    integrator(field, &other_field, &interaction);
                }
            }            
        }
    }
}

