
pub mod cell_auto;
pub mod from_objects;
pub mod variant;
use variant::FieldIntegratorVariant;

// use crate::boundary::field::variant::FieldBoundaryVariant;
// use crate::boundary::field as boundary;
use crate::integrator::field as integrator;


pub fn step(
    field:      &mut crate::state::field::Field,
    states:     &Vec<crate::state::State>,
    config:     &crate::config::EngineConfig,
) {
    // apply integration scheme
    let apply_integrator = match config.fields[field.id].integrator {
        FieldIntegratorVariant::CellAuto    => integrator::cell_auto::step,
        FieldIntegratorVariant::FromObjects => integrator::from_objects::get,
    };
    apply_integrator(field, &states, &config);

    // apply boundaries
    // let apply_boundaries = match config.fields[field.id].boundary {
    //     FieldBoundaryVariant::None          => boundary::none::apply,
    //     FieldBoundaryVariant::Periodic      => boundary::periodic::apply,
    // };
    // apply_boundaries(field, &config.fields[field.id]); // TODO update only rel. cells
}

