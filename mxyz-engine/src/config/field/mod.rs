#![allow(non_snake_case)]

use crate::integrator::field::variant::FieldIntegratorVariant;
use crate::state::field::variant::FieldVariant;
use crate::interaction::field::field::FieldInteraction;
use crate::interaction::field::object::ObjInteraction;


pub struct FieldEngineConfig {

    pub id: usize,
    pub field_variant: FieldVariant,
    pub dimensions: Vec<usize>,
    // matrix for interactions (?)
    // pub initial_state: Vec<f64>,   // TODO: bool / int ?
    pub integrator: FieldIntegratorVariant,
    pub field_interactions: Vec<FieldInteraction>,
    pub obj_interactions: Vec<ObjInteraction>,
    // pub boundary: FieldBoundary,

}
impl FieldEngineConfig {

    pub fn new(
        id: usize, 
        // dt: f64,
    ) -> Self {

        let DEFAULT_FIELD_VARIANT = FieldVariant::Ising;
        let DEFAULT_DIMENSIONS = Vec::from([10, 10, 1]);
        let DEFAULT_INTEGRATOR = FieldIntegratorVariant::RandomBatch;
        let DEFAULT_FIELD_INTERACTIONS = Vec::from([FieldInteraction::Ising]);
        let DEFAULT_OBJ_INTERACTIONS = Vec::new();

        FieldEngineConfig {
            id,
            field_variant: DEFAULT_FIELD_VARIANT,
            dimensions: DEFAULT_DIMENSIONS,
            field_interactions: DEFAULT_FIELD_INTERACTIONS,
            obj_interactions: DEFAULT_OBJ_INTERACTIONS,
            integrator: DEFAULT_INTEGRATOR,
            // integrator
        }
    }
}

