
use crate::simulation::renderer::object::color_mode::ObjectColorMode;
use crate::simulation::renderer::object::tail_variant::ObjectTailVariant;


pub struct ObjFamilyRendererConfig {

    pub id: usize,
    // pub is_displayed: bool,
    pub is_displaying_objects: bool,
    pub is_displaying_line_tail: bool,
    pub is_displaying_area_tail: bool,
    pub is_displaying_pos_vec: bool,
    pub is_displaying_vel_vec: bool,
    pub is_displaying_acc_vec: bool,
    pub is_displaying_center_of_mass: bool,
    pub is_displaying_center_of_momentum: bool,
    // pub display_acc_vec: bool,
    // pub drawing_radius: f64,
    pub color_mode: ObjectColorMode,
    pub tail_variant: ObjectTailVariant,

}
impl ObjFamilyRendererConfig {

    pub fn new(id: usize) -> Self {
        ObjFamilyRendererConfig {
            id: id,
            // is_displayed: true,
            is_displaying_objects: true,
            is_displaying_line_tail: false,
            is_displaying_area_tail: false,
            is_displaying_pos_vec: false,
            is_displaying_vel_vec: false,
            is_displaying_acc_vec: false,
            is_displaying_center_of_mass: false,
            is_displaying_center_of_momentum: false,
            // display_acc_vec: bool,
            // drawing_radius: 0.02,
            color_mode: ObjectColorMode::Default,
            tail_variant: ObjectTailVariant::None,
        }
    }
}

