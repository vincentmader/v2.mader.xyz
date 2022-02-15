
pub mod euler;
pub mod runge_kutta;
pub mod verlet;
pub mod leapfrog;

pub mod variant;
// pub use variant::ObjIntegratorVariant;

use crate::boundary::object::variant::ObjBoundaryVariant;
use crate::config::EngineConfig;
use crate::integrator::object::variant::ObjIntegratorVariant;
use crate::state::State;
use crate::state::object::ObjFamily;
// use crate::interaction;


pub fn step(
    family: &mut ObjFamily,
    states: &Vec<State>,
    config: &EngineConfig,
) {
    let stepper = match config.obj_families[family.id].integrator {
        ObjIntegratorVariant::EulerExplicit => crate::integrator::object::euler::explicit::step,
        // ObjIntegratorVariant::EulerImplicit => crate::integrator::object::euler::implicit::step,
        // ObjIntegratorVariant::RungeKutta2   => crate::integrator::object::runge_kutta::order_2::step,
        // ObjIntegratorVariant::RungeKutta4   => crate::integrator::object::runge_kutta::order_4::step,
        // ObjIntegratorVariant::Verlet        => crate::integrator::object::verlet::step,
        // ObjIntegratorVariant::LeapFrog      => crate::integrator::object::leapfrog::step,
    };
    stepper(family, states, config);

    let bound = match config.obj_families[family.id].boundary {
        ObjBoundaryVariant::None                   => crate::boundary::object::none::apply,
        ObjBoundaryVariant::Periodic               => crate::boundary::object::periodic::apply,
        ObjBoundaryVariant::WallCollisionElastic   => crate::boundary::object::collision::wall::elastic::apply,
        ObjBoundaryVariant::WallCollisionInelastic => crate::boundary::object::collision::wall::inelastic::apply,
    };
    bound(family, &config.obj_families[family.id]);
}

// pub struct ObjIntegrator {

//     pub variant: ObjIntegratorVariant,
//     pub dt: f64,
//     pub field_interactions: Vec<interaction::object::field::Interaction>,
//     pub obj_interactions: Vec<interaction::object::object::Interaction>,
    
// }
// impl ObjIntegrator {

//     pub fn new(

//         variant: ObjIntegratorVariant,
//         dt: f64,
//         field_interactions: Vec<interaction::object::field::Interaction>,
//         obj_interactions: Vec<interaction::object::object::Interaction>,

//     ) -> Self { 

//         ObjIntegrator {
//             variant,
//             dt,
//             field_interactions,
//             obj_interactions,
//         }

//     }

//     pub fn step(
//         &mut self, 
//         iter_idx: usize,
//         family: &mut ObjFamily,
//         states: &Vec<State>,
//         config: &EngineConfig,
//     ) {

//         let stepper = match self.variant {
//             ObjIntegratorVariant::EulerExplicit => euler::explicit::step,
//             // ObjIntegratorVariant::EulerImplicit => euler::implicit::step,
//             // ObjIntegratorVariant::RungeKutta2 => runge_kutta::order_2::step,
//             // ObjIntegratorVariant::RungeKutta4 => runge_kutta::order_4::step,
//             // ObjIntegratorVariant::Verlet => verlet::step,
//             // ObjIntegratorVariant::LeapFrog => leapfrog::step,
//         };
//         stepper(
//             iter_idx,
//             family,
//             states,
//             &self.field_interactions,
//             &self.obj_interactions, 
//             self.dt,
//             // &config,
//         );

//     }

// }

