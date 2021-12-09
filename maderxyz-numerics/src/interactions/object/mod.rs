
// use crate::integrators::object::IntegratorVariant;
pub mod newtonian_gravity;
pub mod lennard_jones;
pub mod coulomb;


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
//             interaction_variant,
//             integrator,
//         }
//     }
// }

#[derive(Clone, PartialEq)]
pub enum InteractionVariant {
    NewtonianGravity,
    // ElasticCollision,
    Coulomb,
    LennardJones,
    // Boid,
    // WallCollision,
}
