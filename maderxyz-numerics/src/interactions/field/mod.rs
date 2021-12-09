
// use crate::integrators::field::IntegratorVariant;
pub mod diffusion;
pub mod spin_spin_interaction;
pub mod game_of_life;


// #[derive(Clone)]
// pub struct Interaction {
//     pub interaction_variant: InteractionVariant,
//     pub integrator: IntegratorVariant,
// }
// impl Interaction {
//     pub fn new(
//         interaction_variant: InteractionVariant,
//         integrator: IntegratorVariant,
//     ) -> Self {
//         Interaction {
//             interaction_variant, integrator
//         }
//     }
// }

#[derive(Clone, PartialEq)]
pub enum InteractionVariant {
    SpinSpin,
    Diffusion,
    GameOfLife,
}

