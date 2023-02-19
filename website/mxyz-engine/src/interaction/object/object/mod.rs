
pub mod boid;
pub mod collision;
pub mod forces;


pub enum ObjObjInteraction {
    // BoidAlignment,
    // BoidCohesion,
    // BoidSeparation,
    // CollisionInelastic,
    // CollisionElastic,
    // EpidemicSIR,
    ForceCoulomb,
    ForceNewtonianGravity,
    ForceLennardJones,
}

