
pub mod entire;
pub mod random_batch;
pub mod cell_auto;

pub mod variant;
use variant::FieldIntegratorVariant;

use crate::boundary::field::variant::FieldBoundaryVariant;
use crate::boundary::field as boundary;
use crate::integrator::field as integrator;


pub fn step(
    field:      &mut crate::state::field::Field,
    states:     &Vec<crate::state::State>,
    config:     &crate::config::EngineConfig,
) {
    // let cell_indices = match self.config.fields[field.id].integrator {
    //     FieldIntegratorVariant::Entire => (0..)
    //     FieldIntegratorVariant::RandomBatch => 
    // };

    let apply_integrator = match config.fields[field.id].integrator {
        FieldIntegratorVariant::CellAuto    => integrator::cell_auto::step,
        // FieldIntegratorVariant::Entire      => integrator::entire::step,
        // FieldIntegratorVariant::RandomBatch => integrator::random_batch::step,
    };
    apply_integrator(field, &states, &config);

    let apply_boundaries = match config.fields[field.id].boundary {
        FieldBoundaryVariant::None          => boundary::none::apply,
        FieldBoundaryVariant::Periodic      => boundary::periodic::apply,
    };
    apply_boundaries(field, &config.fields[field.id]);

}

