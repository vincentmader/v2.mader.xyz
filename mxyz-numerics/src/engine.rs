
use crate::state::State;
use crate::state::object::ObjectVariant;
use crate::integrator::setup::IntegratorSetup;

use crate::integrator::object as object_integrators;
use crate::integrator::field as field_integrators;


pub struct Engine {

    page_id: String,
    iteration_idx: usize,
    integrator_setup: IntegratorSetup,
    pub states: Vec<State>,

}
impl Engine {

    pub fn new(page_id: &str) -> Self {

        let page_id = String::from(page_id);
        let iteration_idx = 0;
        let states = Vec::new();
        let integrator_setup = IntegratorSetup::new();

        Engine { page_id, iteration_idx, integrator_setup, states }
    }

    pub fn init(&mut self) {

        self.states = Vec::from([ 
            State::new(&self.page_id, &mut self.integrator_setup)
        ]);

    }

    pub fn reset(&mut self) {

        self.iteration_idx = 0;
        self.init();

    }

    pub fn step(&mut self) {

        let current_state = &self.states[self.iteration_idx];
        let mut next_state = current_state.clone();

        // utils::dom::console::log(&format!("iteration {}", self.iteration_idx));

        for (idx, field) in next_state.fields.iter_mut().enumerate() {
            let integrators = &mut self.integrator_setup.field[idx];
            for integrator in integrators.iter_mut() {
                integrator.step(field, &self.states);
            }
        }

        for (idx, object_family) in next_state.object_families.iter_mut().enumerate() {
            if matches!(object_family.variant, ObjectVariant::Static) { continue }

            // setup  (TODO generalize)
            let integrators = &mut self.integrator_setup.object[idx];

            // TODO get_neighbors()
            // todo: get relevant pairs: (other_fam_id, other_id) 

            for integrator in integrators.iter_mut() {
                integrator.step(object_family, &self.states); // + neighborhood
            }

            // TODO handle boundaries
            let boundaries = &mut self.integrator_setup.object_boundaries;
            for boundary in boundaries[idx].iter_mut() {
                boundary.apply(object_family);
            }

        }

        self.states.push(next_state);
        self.iteration_idx += 1;

    }

}

