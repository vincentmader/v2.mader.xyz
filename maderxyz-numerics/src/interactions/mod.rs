
pub mod coulomb_interaction;
pub mod newtonian_gravity;


#[derive(Clone, PartialEq)]
pub enum ObjectInteraction {
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
