
use crate::simulation::renderer::object::color_mode::ObjColorMode;
use crate::simulation::renderer::object::tail_variant::ObjTailVariant;


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
    pub color_mode: ObjColorMode,
    pub tail_variant: ObjTailVariant,

}
impl ObjFamilyRendererConfig {

    pub fn new(id: usize) -> Self {

        // let tail_variant = match 


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
            color_mode: ObjColorMode::Default,
            tail_variant: ObjTailVariant::None,
        }
    }
}

