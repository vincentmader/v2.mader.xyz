
use crate::integrators::Integrator;
pub mod coulomb_interaction;
pub mod newtonian_gravity;


#[derive(Clone)]
pub struct ObjectInteraction {
    pub interaction_variant: ObjectInteractionVariant,
    pub integrator: Integrator,
}
impl ObjectInteraction {
    pub fn new(
        interaction_variant: ObjectInteractionVariant,
        integrator: Integrator,
    ) -> Self {
        ObjectInteraction {
            interaction_variant,
            integrator,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ObjectInteractionVariant {
    NewtonianGravity,
    // ElasticCollision,
    CoulombInteraction,
    // Boid,
    // WallCollision,
}

#[derive(Clone, PartialEq)]
pub enum FieldInteraction {
    // SpinFlip,
    Diffusion,
    // GameOfLife,
}
