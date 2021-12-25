
use crate::state::State;
use crate::state::object::ObjectVariant;
use crate::integrator::setup::IntegratorSetup;


pub struct Engine {

    page_id: String,
    pub iteration_idx: usize,
    pub integrator_setup: IntegratorSetup,
    pub states: Vec<State>,

}
impl Engine {

    pub fn new(page_id: &str) -> Self {

        let page_id = String::from(page_id);
        let iteration_idx = 0;
        let integrator_setup = IntegratorSetup::new();
        let states = Vec::new();
        Engine { page_id, iteration_idx, integrator_setup, states }
    }

    pub fn init(&mut self) { 

        self.states = Vec::from([
            State::new(&self.page_id, &mut self.integrator_setup)
        ]);

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

            // setup
            let integrator = &mut self.integrator_setup.field[field_id];
            // integrator.step(self.iteration_idx, field, &self.states);

        }

        let families = &mut state_new.object_families;
        for mut family in families.iter_mut() {
            let family_id = family.id;

            // setup  (TODO generalize)
            let integrator = &mut self.integrator_setup.object[family_id];
            let boundaries = &mut self.integrator_setup.object_boundaries;

            // TODO get relevant neighbor: (other_fam_id, other_id) 

            integrator.step(  // multiple integrators?
                self.iteration_idx, &mut family, &self.states,
            ); // + neighborhood

            boundaries[family.id].apply(  // multiple boundaries?
                &mut family
            );

        }

        self.states.push(state_new);
        self.iteration_idx += 1;
    }
}

