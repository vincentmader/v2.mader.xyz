
use crate::simulation::renderer::object::color_mode::ObjColorMode;
use crate::simulation::renderer::object::tail_variant::ObjTailVariant;

use mxyz_engine::config::obj_family::ObjFamilyEngineConfig;
use mxyz_engine::state::object::variant::ObjVariant;


pub struct ObjFamilyRendererConfig {

    pub id:                                 usize,
    // pub is_displayed: bool,
    pub is_displaying_objects:              bool,
    pub is_displaying_line_tail:            bool,
    pub is_displaying_area_tail:            bool,
    pub is_displaying_pos_vec:              bool,
    pub is_displaying_vel_vec:              bool,
    pub is_displaying_acc_vec:              bool,
    pub is_displaying_center_of_mass:       bool,
    pub is_displaying_center_of_momentum:   bool,
    // pub display_acc_vec:                    bool,
    pub obj_drawing_radius:                 f64,
    pub color_mode:                         ObjColorMode,
    pub tail_variant:                       ObjTailVariant,
    pub tail_length:                        usize,
    pub tail_width:                         f64,
    pub obj_is_filled:                      bool,

}
impl ObjFamilyRendererConfig {

    pub fn new(sim_id: &str, id: usize, obj_fam: &ObjFamilyEngineConfig) -> Self {

        let obj_drawing_radius = match obj_fam.obj_variant {
            ObjVariant::Static   => 0.05,
            ObjVariant::Body     => 0.02,
            ObjVariant::Particle => 0.005,
        };
        let DEFAULT_COLOR_MODE = match sim_id {
            "charge-interaction" => ObjColorMode::Charge,
            "3body-fig8" => ObjColorMode::Speed,
            "nbody-misc" | "nbody-asteroids" | "nbody-flowers" => ObjColorMode::Distance,
            _ => ObjColorMode::Default
        };
        let DEFAULT_TAIL_VARIANT = match sim_id {
            "3body-moon" | "nbody-flowers" | "nbody-misc" | "3body-fig8" => ObjTailVariant::Line,
            // "3body-fig8" => ObjTailVariant::Area,
            _ => ObjTailVariant::None
        };
        let DEFAULT_TAIL_LENGTH = match sim_id {
            "3body-fig8" => 200,
            _ => 100
        };
        let DEFAULT_TAIL_WIDTH = 2.;

        ObjFamilyRendererConfig {
            id,
            // is_displayed:                       true,
            is_displaying_objects:              true,
            is_displaying_line_tail:            false,
            is_displaying_area_tail:            false,
            is_displaying_pos_vec:              false,
            is_displaying_vel_vec:              false,
            is_displaying_acc_vec:              false,
            is_displaying_center_of_mass:       false,
            is_displaying_center_of_momentum:   false,
            // display_acc_vec:                    bool,
            // drawing_radius:                     0.02,
            color_mode:                         DEFAULT_COLOR_MODE,
            tail_variant:                       DEFAULT_TAIL_VARIANT,
            tail_length:                        DEFAULT_TAIL_LENGTH,
            tail_width:                         DEFAULT_TAIL_WIDTH,
            obj_drawing_radius,
            obj_is_filled:                      true,  // TODO add button
        }
    }
}

