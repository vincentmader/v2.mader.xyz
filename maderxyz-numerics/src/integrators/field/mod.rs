
pub mod batch_wise;
pub mod entire;

use crate::interactions::field::InteractionVariant;


#[derive(Clone)]
pub struct Integrator {
    pub variant: IntegratorVariant,
    pub interactions: Vec<InteractionVariant>,
}
impl Integrator {
    pub fn new(
        variant: IntegratorVariant, 
        interactions: Vec<InteractionVariant>
    ) -> Self {
        Integrator {
            variant, 
            interactions
        }
    }
}


#[derive(Clone)]
pub enum IntegratorVariant {
    // TODO 
    BatchWise,
    Entire,
}

