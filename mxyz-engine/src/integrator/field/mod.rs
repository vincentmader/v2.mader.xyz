
// use crate::fields::state::Field;
// use crate::fields::interactions::Interaction;
// use crate::interactions::field::Interaction;
// use crate::state::State;
use crate::state::State;
use crate::state::field::Field;
use crate::interaction::field::field::FieldInteraction;
use crate::interaction::field::object::ObjInteraction;
pub mod entire;
pub mod random_batch;

pub mod variant;
pub use variant::FieldIntegratorVariant;
pub use crate::config::field::FieldEngineConfig;


pub struct FieldIntegrator {

    pub variant: FieldIntegratorVariant,
    pub field_interactions: Vec<FieldInteraction>,
    pub obj_interactions: Vec<ObjInteraction>,
    
}
// impl FieldIntegrator {

//     pub fn new(

//         variant: FieldIntegratorVariant,
//         field_interactions: Vec<FieldInteraction>,
//         obj_interactions: Vec<ObjInteraction>,

//     ) -> Self {

//         FieldIntegrator {
//             variant,
//             field_interactions,
//             obj_interactions,
//         }

//     }

//     pub fn step(
//         &mut self,
//         iter_idx: usize,
//         field: &mut Field,
//         states: &Vec<State>,
//         config: &FieldEngineConfig,
//     ) {

//         let stepper = match self.variant {
//             FieldIntegratorVariant::Entire => entire::step,
//             FieldIntegratorVariant::RandomBatch => random_batch::step,
//         };
//         stepper(iter_idx, field, states, config);

//     }

// }


// pub trait Integrator {
//     fn step(
//         field: &mut Field,
//         states: &Vec<State>,
//         // interactions: &Vec<Interaction>,
//     );
// }

