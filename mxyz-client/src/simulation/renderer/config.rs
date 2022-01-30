
use crate::simulation::renderer::object::color_mode::ObjectColorMode;


pub struct ObjFamilyRendererConfig {
    pub id: usize,
    pub display: bool,
    pub display_objects: bool,
    pub display_line_tail: bool,
    pub display_area_tail: bool,
    pub display_pos_vec: bool,
    pub display_vel_vec: bool,
    // pub display_acc_vec: bool,
    pub drawing_radius: f64,
    pub color_mode: ObjectColorMode,
}

pub struct FieldRendererConfig {
    id: usize,
    display: bool,
    is_paused: bool,
}

pub struct RendererConfig {
    fields: Vec<FieldRendererConfig>,
    obj_families: Vec<ObjFamilyRendererConfig>,
    zoom: f64,
    is_paused: bool,
    clear_canvas: bool,
    display_hud: bool,
}
impl RendererConfig {
    pub fn new() -> Self {
        RendererConfig {
            fields: Vec::new(),
            obj_families: Vec::new(),
            zoom: 1.,
            is_paused: false,
            clear_canvas: true,
            display_hud: false,
        }
    }
}


