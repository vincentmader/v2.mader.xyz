
pub mod field;
pub mod obj_family;

use field::FieldEngineConfig;
use obj_family::ObjFamilyEngineConfig;


pub struct EngineConfig {

    pub fields:         Vec<FieldEngineConfig>,
    pub obj_families:   Vec<ObjFamilyEngineConfig>,
    pub is_paused:      bool,
    pub dt:             f64,

}
impl EngineConfig {

    pub fn new() -> Self {

        const DEFAULT_TIME_STEP_SIZE: f64 = 0.01;

        let fields = Self::init_fields(DEFAULT_TIME_STEP_SIZE);
        let obj_families = Self::init_obj_families(DEFAULT_TIME_STEP_SIZE);
        EngineConfig {
            fields: fields,
            obj_families: obj_families,
            is_paused: false,
            dt: DEFAULT_TIME_STEP_SIZE,
        }
    }

    pub fn init_fields(dt: f64) -> Vec<FieldEngineConfig> {
        let mut fields = Vec::new();

        let id = 0;
        let field = FieldEngineConfig::new(id, dt);
        fields.push(field);

        fields
    }

    pub fn init_obj_families(dt: f64) -> Vec<ObjFamilyEngineConfig> {
        let mut obj_families = Vec::new();

        let id = 0;
        let obj_fam = ObjFamilyEngineConfig::new(id, dt);
        obj_families.push(obj_fam);

        obj_families
    }
}

