
use crate::state::object::variant::ObjVariant;

use crate::interaction::object::object::forces as obj_obj_forces; // TODO rename -> obj_obj_interactions


pub fn get(
    field:  &mut crate::state::field::Field,
    states: &Vec<crate::state::State>,
    config: &crate::config::EngineConfig,
) {
    let iter_idx = config.iter_idx;
    let current_state = &states[iter_idx];
    let epsilon = 0.; // TODO

    let field_conf = &config.fields[field.id];
    let field_dimensions = &field_conf.dimensions;

    for y_idx in 0..field_dimensions[0] {
        for x_idx in 0..field_dimensions[1] {  // TODO generalize to 3D / nD

            let x = (2.*(x_idx as f64 + 0.5) / field_dimensions[0] as f64) - 1.;  // TODO zoom
            let y = (2.*(y_idx as f64 + 0.5) / field_dimensions[1] as f64) - 1.;
            let (m, u, v, q) = (1., 0., 0., 1.);
            let field_obj = [m, x, y, u, v, q];
            let mut force = Vec::from([0., 0.]);

            for obj_family in &current_state.obj_families {
                let family_config = &config.obj_families[obj_family.id];
                if matches!(family_config.obj_variant, ObjVariant::Particle) { continue }
        
                for obj_idx in 0..family_config.family_size {
        
                    let obj_length = family_config.obj_length;
                    let obj_indices = obj_idx*obj_length..(obj_idx+1)*obj_length;
                    let obj = &obj_family.objects[obj_indices];
        
                    let force_getter = match config.sim_id.as_str() {
                        "lennard-jones" => obj_obj_forces::lennard_jones::force,
                        "charge-interaction" => obj_obj_forces::coulomb::force,
                        _ => obj_obj_forces::newtonian_gravity::force,
                    };
                    let f = force_getter(
                        &field_obj, &obj, epsilon,
                    );
                    force[0] += f[0];
                    force[1] += f[1];
                }
            }
        }
    }
}

