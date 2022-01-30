
use wasm_bindgen::prelude::*;

use mxyz_engine::boundary::field::variant::BoundaryVariant as FieldBoundaryVariant;
use mxyz_engine::boundary::object::variant::BoundaryVariant as ObjectBoundaryVariant;
use mxyz_engine::integrator::field::variant::IntegratorVariant as FieldIntegratorVariant;
use mxyz_engine::integrator::object::variant::IntegratorVariant as ObjectIntegratorVariant;
use mxyz_engine::state::object::ObjectVariant;
use crate::simulation::Simulation;
use crate::simulation::renderer::object::color_mode::ObjectColorMode;
use crate::simulation::renderer::object::tail_variant::ObjectTailVariant;


#[wasm_bindgen]
impl Simulation {

    pub fn handle_button_event(&mut self, button_id: &str) {

        // let document = utils::dom::document();
        // let button = document.get_element_by_id(button_id).unwrap();

        let mut rel_button_id = String::from(button_id);
        // get id of field/object_family that button belongs to
        let mut thing_id: usize = 0;
        if button_id.starts_with("obj-fam_") || button_id.starts_with("field_") {
            // id
            let mut foo = button_id.split("_");
            thing_id = foo.nth(1).unwrap().parse::<usize>().unwrap();
            // get button_id without object_family id
            let mut foo = button_id.split("_");
            let count = button_id.split("_").count();
            rel_button_id = String::from("button");
            for idx in 3..count {
                rel_button_id = format!("{}_{}", rel_button_id, foo.nth(idx).unwrap());
            }
        }

        let engine = &mut self.engine;
        let renderer = &mut self.renderer;

        match rel_button_id.as_str() {

            // GENERAL (directly under canvas)

            "button_reset" => {
                engine.init();  // TODO instead: go back to first state, delete rest, use integrator setup
                engine.iteration_idx = 0;
                renderer.reset();
            }, 
            "button_toggle-pause" => {
                renderer.is_paused = !renderer.is_paused;
                self.is_paused = !self.is_paused;
            },
            "button_toggle-pause-engine" => {
                self.is_paused = !self.is_paused;
            },
            "button_toggle-pause-renderer" => {
                renderer.is_paused = !renderer.is_paused;
            },
            "button_toggle-halt-renderer" => {
                renderer.is_halted = !renderer.is_halted;
            },
            "button_toggle-display-hud" => {
                renderer.is_displaying_hud = !renderer.is_displaying_hud;
            },
            "button_toggle-clear-canvas" => {
                renderer.is_clearing_canvas = !renderer.is_clearing_canvas;
            },

            // OBJECT VARIANT

            "button_set-obj-variant-particle" => {
                let iteration_idx = engine.iteration_idx;
                engine.states[iteration_idx].object_families[thing_id].variant = ObjectVariant::Particle;
            },
            "button_set-obj-variant-body" => {
                let iteration_idx = engine.iteration_idx;
                engine.states[iteration_idx].object_families[thing_id].variant = ObjectVariant::Body;
            },
            "button_set-obj-variant-static" => {
                let iteration_idx = engine.iteration_idx;
                engine.states[iteration_idx].object_families[thing_id].variant = ObjectVariant::Static;
            },

            // OBJECT COLOR MODE
            "button_set-obj-col-default" => {
                renderer.obj_color_mode[thing_id] = ObjectColorMode::Default;
            },
            "button_set-obj-col-dist" => {
                renderer.obj_color_mode[thing_id] = ObjectColorMode::Distance;
            },
            "button_set-obj-col-speed" => {
                renderer.obj_color_mode[thing_id] = ObjectColorMode::Speed;
            },
            "button_set-obj-col-mass" => {
                renderer.obj_color_mode[thing_id] = ObjectColorMode::Mass;
            },
            "button_set-obj-col-charge" => {
                renderer.obj_color_mode[thing_id] = ObjectColorMode::Charge;
            },
            "button_set-obj-col-hsv-pos" => {
                renderer.obj_color_mode[thing_id] = ObjectColorMode::HSLPosition;
            },
            "button_set-obj-col-hsv-vel" => {
                renderer.obj_color_mode[thing_id] = ObjectColorMode::HSLVelocity;
            },

            // OBJECT TAIL VARIANT

            "button_set-obj-tail-variant-none" => {
                renderer.obj_tail_variant[thing_id] = ObjectTailVariant::None;
            },
            "button_set-obj-tail-variant-line" => {
                renderer.obj_tail_variant[thing_id] = ObjectTailVariant::Line;
            },
            "button_set-obj-tail-variant-area" => {
                renderer.obj_tail_variant[thing_id] = ObjectTailVariant::Area;
            },

            // OBJECT MOTION VECTORS

            "button_toggle-display-obj-vec-pos" => {
                renderer.is_drawing_pos_vec[thing_id] = !renderer.is_drawing_pos_vec[thing_id]
            },
            "button_toggle-display-obj-vec-vel" => {
                renderer.is_drawing_vel_vec[thing_id] = !renderer.is_drawing_vel_vec[thing_id]
            },
            "button_toggle-display-obj-vec-acc" => {
                renderer.is_drawing_acc_vec[thing_id] = !renderer.is_drawing_acc_vec[thing_id]
            },

            // FIELD VARIANT

            // ...

            // INTEGRATOR VARIANT

            "button_set-obj-integrator-euler-exp" => {
                engine.engine_setup.object_integrators[thing_id].variant = ObjectIntegratorVariant::EulerExplicit;
            },
            "button_set-obj-integrator-euler-imp" => {
                engine.engine_setup.object_integrators[thing_id].variant = ObjectIntegratorVariant::EulerImplicit;
            },
            "button_set-obj-integrator-rk2" => {
                engine.engine_setup.object_integrators[thing_id].variant = ObjectIntegratorVariant::RungeKutta2;
            },
            "button_set-obj-integrator-rk4" => {
                engine.engine_setup.object_integrators[thing_id].variant = ObjectIntegratorVariant::RungeKutta4;
            },
            "button_set-obj-integrator-leapfrog" => {
                engine.engine_setup.object_integrators[thing_id].variant = ObjectIntegratorVariant::LeapFrog;
            },
            "button_set-obj-integrator-verlet" => {
                engine.engine_setup.object_integrators[thing_id].variant = ObjectIntegratorVariant::Verlet;
            },

            // BOUNDARY VARIANT

            "button_set-obj-bound-none" => {  // TODO
                self.engine.engine_setup.object_boundaries[thing_id].variant = ObjectBoundaryVariant::None;
            },
            "button_set-obj-bound-periodic" => {
                engine.engine_setup.object_boundaries[thing_id].variant = ObjectBoundaryVariant::Periodic;
            },
            "button_set-obj-bound-wall-elastic" => {
                engine.engine_setup.object_boundaries[thing_id].variant = ObjectBoundaryVariant::WallCollisionElastic;
            },
            "button_set-obj-bound-wall-inelastic" => {
                engine.engine_setup.object_boundaries[thing_id].variant = ObjectBoundaryVariant::WallCollisionInelastic;
            },

            // ...

            "button_set-obj-interaction-none" => {
                // let nr_of_families = self.engine.states[self.engine.iteration_idx].object_families.len();
                // for idx in 0..nr_of_families {
                //     // self.engine.engine_setup.object[0].object_interactions;
                // }
                // TODO save interactions not in enum, but struct
                    // e.g.     grav: true, coulomb: true, lj: false, ...
                    //     -> nope!  (?) },
            }, _ => { mxyz_utils::dom::console::log("ERROR: button not found"); }
        };
        mxyz_utils::dom::console::log(&format!("{}", button_id));
    }

}

