
use std::collections::HashMap;

use crate::integrators;
use crate::integrators::object::IntegratorVariant as ObjectIntegratorVariant;
use crate::integrators::field::IntegratorVariant as FieldIntegratorVariant;
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

            // loop over integrators
            for integrator in object_family.integrators.clone().iter() {  // TODO get rid of clone
                let f = match integrator.variant {
                    ObjectIntegratorVariant::EulerExplicit => integrators::object::euler_explicit::step,
                    // ObjectIntegratorVariant::EulerImplicit => integrators::object::euler_implicit::step,
                    // ObjectIntegratorVariant::RungeKutta2 => integrators::object::runge_kutta_2::step,
                    // ObjectIntegratorVariant::RungeKutta4 => integrators::object::runge_kutta_4::step,
                    // ObjectIntegratorVariant::LeapFrog => integrators::object::leap_frog::step,
                    // ObjectIntegratorVariant::Verlet => integrators::object::verlet::step,
                };
                // loop over object families (including this one)
                for other_family in current_state.object_families.iter() {
                    // don't apply influence of low-mass particles
                    if matches!(other_family.object_type, ObjectType::Particle) { continue };

                    f(object_family, other_family, &integrator.interactions);
                }
            }
        }
    }

    fn step_fields(next_state: &mut State, current_state: &State) {

        // loop over fields
        for (field_idx, field) in next_state.fields.iter_mut().enumerate() {
            // loop over integrators
            for integrator in current_state.fields[field_idx].integrators.iter() {
                let f = match integrator.variant {
                    FieldIntegratorVariant::BatchWise => integrators::field::batch_wise::step,
                    FieldIntegratorVariant::Entire => integrators::field::entire::step,
                };

                // loop over fields (including this one)
                for other_field in current_state.fields.iter() {
                    // // only apply interactions when both fields "feel" them
                    // for other_interaction in other_field.interactions.iter() {
                    //     if interaction.interaction_variant != other_interaction.interaction_variant { continue }
                    f(field, &other_field, &integrator.interactions);
                }
            }
        }
    }
}

