
pub mod field;
pub mod obj_family;

use field::FieldRendererConfig;
use obj_family::ObjFamilyRendererConfig;


pub struct RendererConfig {

    pub fields:                 Vec<FieldRendererConfig>,
    pub obj_families:           Vec<ObjFamilyRendererConfig>,
    pub zoom:                   f64,
    pub is_paused:              bool,
    pub is_clearing_canvas:     bool,
    pub is_iterating_forward:   bool,
    pub is_displaying_hud:      bool,

}
impl RendererConfig {

    pub fn new(
        // sim_id: &str
    ) -> Self {
        RendererConfig {
            fields: Self::init_fields(),
            obj_families: Self::init_obj_families(),
            zoom: 1.,
            is_paused: false,
            is_clearing_canvas: true,
            is_iterating_forward: true,
            is_displaying_hud: true,
        }
    }

    pub fn init_fields() -> Vec<FieldRendererConfig> {
        let mut fields = Vec::new();

        let id = 0;
        let field = FieldRendererConfig::new(id);
        fields.push(field);

        fields
    }

    pub fn init_obj_families() -> Vec<ObjFamilyRendererConfig> {
        let mut obj_families = Vec::new();

        let id = 0;
        let obj_fam = ObjFamilyRendererConfig::new(id);
        obj_families.push(obj_fam);

        let id = 1;
        let obj_fam = ObjFamilyRendererConfig::new(id);
        obj_families.push(obj_fam);

        obj_families
    }
}


