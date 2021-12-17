
// use crate::fields::state::Field;
// use crate::fields::interactions::Interaction;
// use crate::interactions::field::Interaction;
// use crate::state::State;
use crate::state::State;
use crate::state::field::Field;
use crate::interaction::field::field::Interaction as FieldInteraction;
use crate::interaction::field::object::Interaction as ObjectInteraction;
pub mod entire;
pub mod random_batch;

mod variant;
pub use variant::IntegratorVariant;


pub struct FieldIntegrator {

    pub variant: IntegratorVariant,
    pub field_interactions: Vec<FieldInteraction>,
    pub object_interactions: Vec<ObjectInteraction>,
    
}
impl FieldIntegrator {

    pub fn new(

        variant: IntegratorVariant,
        field_interactions: Vec<FieldInteraction>,
        object_interactions: Vec<ObjectInteraction>,

    ) -> Self {

        FieldIntegrator {
            variant,
            field_interactions,
            object_interactions,
        }

    }

    pub fn step(
        &mut self,
        field: &mut Field,
        states: &Vec<State>,
    ) {

        let stepper = match self.variant {
            IntegratorVariant::Entire => random_batch::step,
            IntegratorVariant::RandomBatch => random_batch::step,
        };
        stepper(field, states);

    }

}


// pub trait Integrator {
//     fn step(
//         field: &mut Field,
//         states: &Vec<State>,
//         // interactions: &Vec<Interaction>,
//     );
// }

