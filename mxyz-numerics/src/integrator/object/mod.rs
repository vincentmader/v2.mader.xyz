
pub mod euler;
pub mod runge_kutta;
pub mod verlet;
pub mod leapfrog;

mod variant;
pub use variant::IntegratorVariant;

use crate::state::State;
use crate::state::object::ObjectFamily;

use crate::interaction;


pub struct ObjectIntegrator {

    pub variant: IntegratorVariant,
    pub dt: f64,
    pub field_interactions: Vec<interaction::object::field::Interaction>,
    pub object_interactions: Vec<interaction::object::object::Interaction>,
    
}
impl ObjectIntegrator {

    pub fn new(

        variant: IntegratorVariant,
        dt: f64,
        field_interactions: Vec<interaction::object::field::Interaction>,
        object_interactions: Vec<interaction::object::object::Interaction>,

    ) -> Self { 

        ObjectIntegrator {
            variant,
            dt,
            field_interactions,
            object_interactions,
        }

    }

    pub fn step(
        &mut self, 
        object_family: &mut ObjectFamily,
        states: &Vec<State>
    ) {

        let stepper = match self.variant {
            IntegratorVariant::EulerExplicit => euler::explicit::step,
            IntegratorVariant::EulerImplicit => euler::implicit::step,
            IntegratorVariant::RungeKutta2 => runge_kutta::order_2::step,
            IntegratorVariant::RungeKutta4 => runge_kutta::order_4::step,
            IntegratorVariant::Verlet => verlet::step,
            IntegratorVariant::LeapFrog => leapfrog::step,
        };
        stepper(
            object_family, 
            states, 
            &self.field_interactions,
            &self.object_interactions, 
            self.dt,
        );

    }

}

