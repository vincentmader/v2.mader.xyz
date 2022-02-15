
pub mod euler;
pub mod runge_kutta;
pub mod verlet;
pub mod leapfrog;

pub mod variant;
use variant::ObjIntegratorVariant;

use crate::boundary::object::variant::ObjBoundaryVariant;
use crate::boundary::object as bounds;


pub fn step(
    family:     &mut crate::state::object::ObjFamily,
    states:     &Vec<crate::state::State>,
    config:     &crate::config::EngineConfig,
) {
    let apply_integrator = match config.obj_families[family.id].integrator {
        ObjIntegratorVariant::EulerExplicit => euler::explicit::step,
        // ObjIntegratorVariant::EulerImplicit => euler::implicit::step,
        // ObjIntegratorVariant::RungeKutta2   => runge_kutta::order_2::step,
        // ObjIntegratorVariant::RungeKutta4   => runge_kutta::order_4::step,
        // ObjIntegratorVariant::Verlet        => verlet::step,
        // ObjIntegratorVariant::LeapFrog      => leapfrog::step,
        // ObjIntegratorVariant::Rails         => rails::step,
    };
    apply_integrator(family, states, config);

    let apply_boundaries = match config.obj_families[family.id].boundary {
        ObjBoundaryVariant::None                   => bounds::none::apply,
        ObjBoundaryVariant::Periodic               => bounds::periodic::apply,
        ObjBoundaryVariant::WallCollisionElastic   => bounds::collision::wall::elastic::apply,
        ObjBoundaryVariant::WallCollisionInelastic => bounds::collision::wall::inelastic::apply,
    };
    apply_boundaries(family, &config.obj_families[family.id]);
}

