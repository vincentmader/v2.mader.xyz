
use crate::integrators::object::Integrator;
pub mod newtonian_gravity;
pub mod coulomb_interaction;


#[derive(Clone)]
pub struct Interaction {
    pub interaction_variant: InteractionVariant,
    pub integrator: Integrator,
}
impl Interaction {
    pub fn new(
        interaction_variant: InteractionVariant,
        integrator: Integrator,
    ) -> Self {
        Interaction {
            interaction_variant,
            integrator,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum InteractionVariant {
    NewtonianGravity,
    // ElasticCollision,
    CoulombInteraction,
    // Boid,
    // WallCollision,
}
