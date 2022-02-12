
// use crate::fields::state::Field;
// use crate::fields::interactions::Interaction;
// use crate::interactions::field::Interaction;
// use crate::state::State;
use crate::state::State;
use crate::state::field::Field;
use crate::interaction::field::field::Interaction as FieldInteraction;
use crate::interaction::field::object::Interaction as ObjInteraction;
pub mod entire;
pub mod random_batch;

pub mod variant;
pub use variant::IntegratorVariant;


pub struct FieldIntegrator {

    pub variant: IntegratorVariant,
    pub field_interactions: Vec<FieldInteraction>,
    pub obj_interactions: Vec<ObjInteraction>,
    
}
impl FieldIntegrator {

    pub fn new(

        variant: IntegratorVariant,
        field_interactions: Vec<FieldInteraction>,
        obj_interactions: Vec<ObjInteraction>,

    ) -> Self {

        FieldIntegrator {
            variant,
            field_interactions,
            obj_interactions,
        }

    }

    pub fn step(
        &mut self,
        iteration_idx: usize,
        field: &mut Field,
        states: &Vec<State>,
    ) {

        let stepper = match self.variant {
            IntegratorVariant::Entire => entire::step,
            IntegratorVariant::RandomBatch => random_batch::step,
        };
        stepper(iteration_idx, field, states);

    }

}


// pub trait Integrator {
//     fn step(
//         field: &mut Field,
//         states: &Vec<State>,
//         // interactions: &Vec<Interaction>,
//     );
// }

