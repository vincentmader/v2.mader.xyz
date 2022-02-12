
pub mod euler;
pub mod runge_kutta;
pub mod verlet;
pub mod leapfrog;

pub mod variant;
// pub use variant::ObjIntegratorVariant;

// use crate::state::State;
// use crate::state::object::family::ObjFamily;

// use crate::interaction;
// use crate::config::EngineConfig;


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
//         iteration_idx: usize,
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
//             iteration_idx,
//             family,
//             states,
//             &self.field_interactions,
//             &self.obj_interactions, 
//             self.dt,
//             // &config,
//         );

//     }

// }

