#![allow(non_snake_case)]

// use crate::integrator::object::ObjIntegrator;
// use crate::integrator::object::IntegratorVariant as ObjIntegratorVariant;
use crate::state::object::variant::ObjVariant;
use crate::state::object::attribute::ObjAttribute;
// use crate::boundary::object::ObjBoundary;
use crate::boundary::object::variant::ObjBoundaryVariant;
use crate::integrator::object::variant::ObjIntegratorVariant;

use crate::interaction::object::field::ObjFieldInteraction;
use crate::interaction::object::object::ObjObjInteraction;


pub struct ObjFamilyEngineConfig {

    pub id:                 usize,
    // matrix for interactions (?)
    // pub family_size: usize,
    pub family_size:        usize,
    pub obj_variant:        ObjVariant,
    pub obj_attributes:     Vec<ObjAttribute>,
    pub integrator:         ObjIntegratorVariant,
    pub field_interactions: Vec<ObjFieldInteraction>,
    pub obj_interactions:   Vec<ObjObjInteraction>,
    pub boundary:           ObjBoundaryVariant,
    pub obj_length:         usize,

}
impl ObjFamilyEngineConfig {

    pub fn new(id: usize) -> Self {

        let DEFAULT_ATTRIBUTES = Vec::from([
            ObjAttribute::Mass,
            ObjAttribute::PosX,
            ObjAttribute::PosY,
            ObjAttribute::VelX,
            ObjAttribute::VelY,
            // ObjAttribute::Charge,
        ]);
        let DEFAULT_OBJ_LENGTH = DEFAULT_ATTRIBUTES.len();

        let DEFAULT_OBJ_INTERACTIONS = Vec::from([
            ObjObjInteraction::ForceNewtonianGravity,
        ]);
        let DEFAULT_FIELD_INTERACTIONS = Vec::from([]);

        let DEFAULT_INTEGRATOR_VARIANT = ObjIntegratorVariant::EulerExplicit;
        // let DEFAULT_INTEGRATOR = ObjIntegrator::new(
        //     DEFAULT_INTEGRATOR_VARIANT, dt, DEFAULT_FIELD_INTERACTIONS, DEFAULT_OBJ_INTERACTIONS,
        // );

        let DEFAULT_BOUNDARY_VARIANT = ObjBoundaryVariant::None;
        // let DEFAULT_BOUNDARY = ObjBoundary::new(
        //     DEFAULT_BOUNDARY_VARIANT,
        // );
    
        ObjFamilyEngineConfig {
            id,
            family_size:        0,  // TODO increment on obj-add
            obj_variant:        ObjVariant::Body,
            obj_attributes:     DEFAULT_ATTRIBUTES,
            integrator:         DEFAULT_INTEGRATOR_VARIANT,
            field_interactions: DEFAULT_FIELD_INTERACTIONS,
            obj_interactions:   DEFAULT_OBJ_INTERACTIONS,
            boundary:           DEFAULT_BOUNDARY_VARIANT,
            obj_length:         DEFAULT_OBJ_LENGTH,
        }
    }
}

