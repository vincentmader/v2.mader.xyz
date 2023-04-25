
pub mod euler;
pub mod runge_kutta;
pub mod verlet;
pub mod leapfrog;

pub mod variant;
use variant::ObjIntegratorVariant;

use crate::boundary::object::variant::ObjBoundaryVariant;
use crate::boundary::object as boundary;
use crate::integrator::object as integrator;


pub fn step(
    family:     &mut crate::state::object::ObjFamily,
    states:     &Vec<crate::state::State>,
    config:     &crate::config::EngineConfig,
) {
    let apply_integrator = match config.obj_families[family.id].integrator {
        ObjIntegratorVariant::EulerExplicit => integrator::euler::explicit::step,
        // ObjIntegratorVariant::EulerImplicit => integrator::euler::implicit::step,
        // ObjIntegratorVariant::RungeKutta2   => integrator::runge_kutta::order_2::step,
        // ObjIntegratorVariant::RungeKutta4   => integrator::runge_kutta::order_4::step,
        // ObjIntegratorVariant::Verlet        => integrator::verlet::step,
        // ObjIntegratorVariant::LeapFrog      => integrator::leapfrog::step,
        // ObjIntegratorVariant::Rails         => integrator::rails::step,
    };
    apply_integrator(family, states, config);

    let apply_boundaries = match config.obj_families[family.id].boundary {
        ObjBoundaryVariant::None                   => boundary::none::apply,
        ObjBoundaryVariant::Periodic               => boundary::periodic::apply,
        ObjBoundaryVariant::WallCollisionElastic   => boundary::collision::wall::elastic::apply,
        ObjBoundaryVariant::WallCollisionInelastic => boundary::collision::wall::inelastic::apply,
    };
    apply_boundaries(family, &config.obj_families[family.id]);
}

