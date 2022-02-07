
use crate::integrator::object::ObjectIntegrator;
use crate::integrator::object::IntegratorVariant as ObjectIntegratorVariant;
use crate::state::object::variant::ObjectVariant;
use crate::state::object::ObjectAttribute;
use crate::boundary::object::ObjectBoundary;

use crate::interaction::object as object_interactions;
use crate::interaction::field as field_interactions;


pub struct ObjFamilyEngineConfig {

    pub id:             usize,
    // matrix for interactions (?)
    // pub family_size: usize,
    pub obj_variant:    ObjectVariant,
    pub obj_attributes: Vec<ObjectAttribute>,
    integrators:        Vec<ObjectIntegrator>,
    // pub boundary:    ObjectBoundary,
    // pub initial_state: Vec<f64>,

}
impl ObjFamilyEngineConfig {

    pub fn new(id: usize, dt: f64) -> Self {

        let DEFAULT_ATTRIBUTES = Vec::from([
            ObjectAttribute::Mass,
            ObjectAttribute::PositionX,
            ObjectAttribute::PositionY,
            ObjectAttribute::VelocityX,
            ObjectAttribute::VelocityY,
        ]);
        let DEFAULT_INTEGRATOR_VARIANT = ObjectIntegratorVariant::EulerExplicit;
        let DEFAULT_OBJ_INTERACTIONS = Vec::from([
            object_interactions::object::Interaction::ForceNewtonianGravity,
        ]);
        let DEFAULT_FIELD_INTERACTIONS = Vec::from([]);
        let DEFAULT_INTEGRATOR = ObjectIntegrator::new(
            DEFAULT_INTEGRATOR_VARIANT, dt, DEFAULT_FIELD_INTERACTIONS, DEFAULT_OBJ_INTERACTIONS,
        );
    
        ObjFamilyEngineConfig {
            id: id,
            obj_variant: ObjectVariant::Body,
            obj_attributes: DEFAULT_ATTRIBUTES,
            integrators: Vec::from([DEFAULT_INTEGRATOR]),
            // family_size: family_size
        }
    }
}
