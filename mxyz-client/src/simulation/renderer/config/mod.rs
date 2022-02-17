
pub mod field;
pub mod obj_family;


pub struct RendererConfig {

    pub fields:                 Vec<field::FieldRendererConfig>,
    pub obj_families:           Vec<obj_family::ObjFamilyRendererConfig>,
    pub zoom:                   f64,
    pub is_paused:              bool,
    pub is_clearing_canvas:     bool,
    pub is_iterating_forward:   bool,
    pub is_displaying_hud:      bool,
    pub sim_id:                 String,

}

impl RendererConfig {

    pub fn new(sim_id: &str) -> Self {
        RendererConfig {
            sim_id:                 String::from(sim_id),
            fields:                 Vec::new(),
            obj_families:           Vec::new(),
            zoom:                   1.,
            is_paused:              false,
            is_clearing_canvas:     true,
            is_iterating_forward:   true,
            is_displaying_hud:      true,
        }
    }

    pub fn init(&mut self, engine: &mxyz_engine::Engine) {
        self.init_fields(engine);
        self.init_obj_families(engine);
    }

    pub fn init_fields(&mut self, engine: &mxyz_engine::Engine) {
        for (id, field) in engine.config.fields.iter().enumerate() {
            let field_conf = field::FieldRendererConfig::new(id, field);
            self.fields.push(field_conf);
        }
    }

    pub fn init_obj_families(&mut self, engine: &mxyz_engine::Engine) {
        for (id, obj_fam) in engine.config.obj_families.iter().enumerate() {
            let obj_fam_conf = obj_family::ObjFamilyRendererConfig::new(id, obj_fam);
            self.obj_families.push(obj_fam_conf);
        }
    }
}

