#![allow(non_snake_case)]

use crate::integrator::field::variant::FieldIntegratorVariant;
use crate::state::field::variant::FieldVariant;
use crate::interaction::field::field::FieldFieldInteraction;
use crate::interaction::field::object::FieldObjInteraction;
use crate::boundary::field::variant::FieldBoundaryVariant;


pub struct FieldEngineConfig {

    pub id:                 usize,
    pub field_variant:      FieldVariant,
    pub dimensions:         Vec<usize>,
    pub integrator:         FieldIntegratorVariant,
    pub field_interactions: Vec<FieldFieldInteraction>,
    pub obj_interactions:   Vec<FieldObjInteraction>,
    pub boundary:           FieldBoundaryVariant,

}
impl FieldEngineConfig {

    pub fn new(
        id: usize, 
        // dt: f64,
    ) -> Self {

        let DEFAULT_FIELD_VARIANT       = FieldVariant::Ising;
        let DEFAULT_DIMENSIONS          = Vec::from([10, 10, 1]);
        let DEFAULT_INTEGRATOR          = FieldIntegratorVariant::RandomBatch;
        let DEFAULT_FIELD_INTERACTIONS  = Vec::from([FieldFieldInteraction::Ising]);
        let DEFAULT_OBJ_INTERACTIONS    = Vec::new();
        let DEFAULT_BOUNDARY_VARIANT    = FieldBoundaryVariant::None;

        FieldEngineConfig {
            id,
            field_variant:      DEFAULT_FIELD_VARIANT,
            dimensions:         DEFAULT_DIMENSIONS,
            field_interactions: DEFAULT_FIELD_INTERACTIONS,
            obj_interactions:   DEFAULT_OBJ_INTERACTIONS,
            integrator:         DEFAULT_INTEGRATOR,
            boundary:           DEFAULT_BOUNDARY_VARIANT,
        }
    }
}

