#![allow(non_snake_case)]


// use crate::integrator::field::FieldIntegrator;
use crate::integrator::field::variant::FieldIntegratorVariant;
use crate::boundary::field::variant::FieldBoundaryVariant;
use crate::interaction::field as field_interactions;


pub struct FieldEngineConfig {

    pub id:                 usize,
    pub integrator:         FieldIntegratorVariant,
    pub field_interactions: Vec<field_interactions::field::Interaction>,
    pub obj_interactions:   Vec<field_interactions::object::Interaction>,
    pub boundary:           FieldBoundaryVariant,
    pub dimensions:         Vec<usize>,

}
impl FieldEngineConfig {

    pub fn new(id: usize) -> Self {

        const DEFAULT_INTEGRATOR_VARIANT: FieldIntegratorVariant = FieldIntegratorVariant::Entire;
        const DEFAULT_BOUNDARY_VARIANT: FieldBoundaryVariant = FieldBoundaryVariant::None;
        let DEFAULT_OBJ_INTERACTIONS = Vec::from([]);
        let DEFAULT_FIELD_INTERACTIONS = Vec::from([]);

        FieldEngineConfig {
            id,
            integrator:         DEFAULT_INTEGRATOR_VARIANT,
            dimensions:         Vec::from([5, 5]),
            obj_interactions:   DEFAULT_OBJ_INTERACTIONS,
            field_interactions: DEFAULT_FIELD_INTERACTIONS,
            boundary:           DEFAULT_BOUNDARY_VARIANT,
        }
    }
}

