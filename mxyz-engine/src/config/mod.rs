
pub mod field;
pub mod obj_family;

use field::FieldEngineConfig;
use obj_family::ObjFamilyEngineConfig;


pub struct EngineConfig {

    pub sim_id:         String,
    pub fields:         Vec<FieldEngineConfig>,
    pub obj_families:   Vec<ObjFamilyEngineConfig>,
    pub is_paused:      bool,
    pub dt:             f64,
    pub iter_idx:       usize,

}
impl EngineConfig {

    pub fn new(sim_id: &str) -> Self {

        const DEFAULT_TIME_STEP_SIZE: f64 = 0.01;

        let fields = Vec::new();
        let obj_families = Vec::new();

        EngineConfig {
            sim_id:         String::from(sim_id),
            fields:         fields,
            obj_families:   obj_families,
            is_paused:      false,
            dt:             DEFAULT_TIME_STEP_SIZE,
            iter_idx:       0,
        }
    }
}

