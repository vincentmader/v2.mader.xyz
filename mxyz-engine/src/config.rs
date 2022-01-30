
use crate::state::object::variant::ObjectVariant;
use crate::integrator::object::ObjectIntegrator;
use crate::boundary::object::ObjectBoundary;


pub struct ObjFamilyEngineConfig {
    pub id: usize,
    // matrix for interactions (?)
    pub family_size: usize,
    pub object_variant: ObjectVariant,
    pub integrator: ObjectIntegrator,
    pub boundary: ObjectBoundary,
    pub initial_state: Vec<f64>,
}

pub struct FieldEngineConfig {
    pub id: usize,
    pub field_resolution: Vec<usize>,
    // matrix for interactions (?)
    pub initial_state: Vec<f64>,   // TODO: bool / int ?
}

pub struct EngineConfig {
    pub fields: Vec<FieldEngineConfig>,
    pub obj_families: Vec<ObjFamilyEngineConfig>,
    pub is_paused: bool,
}
impl EngineConfig {
    pub fn new() -> Self {
        EngineConfig {
            fields: Vec::new(),
            obj_families: Vec::new(),
            is_paused: false,
        }
    }
}

