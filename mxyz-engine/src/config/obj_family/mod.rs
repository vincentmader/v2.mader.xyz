#![allow(non_snake_case)]


use crate::boundary::object::variant::ObjBoundaryVariant;
use crate::integrator::object::variant::ObjIntegratorVariant;
use crate::interaction::object as obj_interactions;
use crate::state::object::attribute::ObjAttribute;
use crate::state::object::variant::ObjVariant;


pub struct ObjFamilyEngineConfig {

    pub id:                 usize,
    pub integrator:         ObjIntegratorVariant,
    pub field_interactions: Vec<obj_interactions::field::Interaction>,
    pub obj_interactions:   Vec<obj_interactions::object::Interaction>,
    pub boundary:           ObjBoundaryVariant,
    pub family_size:        usize,
    pub obj_variant:        ObjVariant,
    pub obj_attributes:     Vec<ObjAttribute>,
    pub obj_length:         usize,

}
impl ObjFamilyEngineConfig {

    pub fn new(id: usize) -> Self {

        const DEFAULT_INTEGRATOR_VARIANT: ObjIntegratorVariant = ObjIntegratorVariant::EulerExplicit;
        const DEFAULT_BOUNDARY_VARIANT: ObjBoundaryVariant = ObjBoundaryVariant::None;
        let DEFAULT_ATTRIBUTES = Vec::from([
            ObjAttribute::Mass,
            ObjAttribute::PositionX,
            ObjAttribute::PositionY,
            ObjAttribute::VelocityX,
            ObjAttribute::VelocityY,
        ]);
        let DEFAULT_OBJ_LENGTH = DEFAULT_ATTRIBUTES.len();
        let DEFAULT_OBJ_INTERACTIONS = Vec::from([
            obj_interactions::object::Interaction::ForceNewtonianGravity,
        ]);
        let DEFAULT_FIELD_INTERACTIONS = Vec::from([]);
    
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

    // TODO update obj_length on attribute add/remove
}

