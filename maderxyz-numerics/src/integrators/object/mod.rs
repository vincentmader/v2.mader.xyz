
pub mod euler_explicit;
pub mod euler_implicit;
pub mod runge_kutta_2;
pub mod runge_kutta_4;
pub mod leap_frog;
pub mod verlet;


use crate::interactions::object::InteractionVariant;


                    // // only apply interactions when both families "feel" them
                    // for other_interaction in other_family.interactions.iter() {
                    //     if interaction.interaction_variant != other_interaction.interaction_variant { continue }
                    // }


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
    EulerExplicit,
    // EulerImplicit,
    // RungeKutta2,
    // RungeKutta4,
    // LeapFrog,
    // Verlet,
}

