#![allow(non_snake_case)]

use crate::integrator::object::ObjectIntegrator;
use crate::integrator::object::IntegratorVariant as ObjectIntegratorVariant;
use crate::state::object::variant::ObjectVariant;
use crate::state::object::ObjectAttribute;
use crate::boundary::object::ObjectBoundary;
use crate::boundary::object::variant::BoundaryVariant as ObjectBoundaryVariant;

use crate::interaction::object as object_interactions;
use crate::interaction::field as field_interactions;


pub struct ObjFamilyEngineConfig {

    pub id:             usize,
    // matrix for interactions (?)
    // pub family_size: usize,
    pub family_size:    usize,
    pub obj_variant:    ObjectVariant,
    pub obj_attributes: Vec<ObjectAttribute>,
    pub integrators:    Vec<ObjectIntegrator>,
    pub boundaries:     Vec<ObjectBoundary>,
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
        let DEFAULT_OBJ_INTERACTIONS = Vec::from([
            object_interactions::object::Interaction::ForceNewtonianGravity,
        ]);
        let DEFAULT_FIELD_INTERACTIONS = Vec::from([]);

        let DEFAULT_INTEGRATOR_VARIANT = ObjectIntegratorVariant::EulerExplicit;
        let DEFAULT_INTEGRATOR = ObjectIntegrator::new(
            DEFAULT_INTEGRATOR_VARIANT, dt, DEFAULT_FIELD_INTERACTIONS, DEFAULT_OBJ_INTERACTIONS,
        );

        mxyz_utils::dom::console::log(&format!("{}", "hallooooooooooo"));

        let DEFAULT_BOUNDARY_VARIANT = ObjectBoundaryVariant::None;
        let DEFAULT_BOUNDARY = ObjectBoundary::new(
            DEFAULT_BOUNDARY_VARIANT,
        );
    
        ObjFamilyEngineConfig {
            id: id,
            family_size:        0,  // TODO increment on obj-add
            obj_variant:        ObjectVariant::Body,
            obj_attributes:     DEFAULT_ATTRIBUTES,
            integrators:        Vec::from([DEFAULT_INTEGRATOR]),
            boundaries:         Vec::from([DEFAULT_BOUNDARY])
        }
    }
}

