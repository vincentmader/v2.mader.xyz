use crate::state::object::variant::ObjVariant;
use crate::state::object::ObjFamily;
use crate::state::State;

use crate::interaction::object::object::forces;
// use crate::interaction::object::field::ObjFieldInteraction;
use crate::interaction::object::object::ObjObjInteraction;

use crate::config::EngineConfig;

pub fn step(family: &mut ObjFamily, states: &Vec<State>, config: &EngineConfig) {
    // skip if object-variant is "static"
    let obj_variant = &config.obj_families[family.id].obj_variant;
    if matches!(obj_variant, ObjVariant::Static) {
        return ();
    }
    // get numerical parameters
    let dt = config.dt;
    let epsilon = 0.05; // TODO: get from obj family? (& saved externally?)
    let iter_idx = config.iter_idx;
    // get info about family from config
    let fam_conf = &config.obj_families[family.id];
    let obj_length = fam_conf.obj_length;
    let obj_interactions = &fam_conf.obj_interactions;
    // loop over objects
    for obj_idx in 0..fam_conf.family_size {
        // get slice representing object
        let obj_indices = obj_idx * obj_length..(obj_idx + 1) * obj_length;
        let obj_slice = &mut family.objects[obj_indices];
        // loop over families
        for other_family in &states[iter_idx].obj_families {
            // skip if other-object-variant is "particle"
            let other_variant = &config.obj_families[other_family.id].obj_variant;
            if matches!(other_variant, ObjVariant::Particle) {
                continue;
            }

            // TODO get relevant neighbor: tree / sectors ?

            // get info about other family from config
            let other_conf = &config.obj_families[other_family.id];
            let other_length = other_conf.obj_length;
            // loop over objects   TODO loop from 0 to obj_idx, update both bodies!
            for other_idx in 0..other_conf.family_size {
                // prevent self-interaction
                if family.id == other_family.id {
                    if obj_idx == other_idx {
                        continue;
                    }
                }
                // get slice representing other object in state vec
                let other_indices = other_idx * other_length..(other_idx + 1) * other_length;
                let other_slice = &other_family.objects[other_indices];
                // loop over interactions
                for interaction in obj_interactions.iter() {
                    // TODO check if both fams have this interaction
                    // TODO handle forces different from other interactions

                    // calculate force
                    let force_getter = match interaction {
                        ObjObjInteraction::ForceNewtonianGravity => {
                            forces::newtonian_gravity::force
                        }
                        ObjObjInteraction::ForceCoulomb => forces::coulomb::force,
                        ObjObjInteraction::ForceLennardJones => forces::lennard_jones::force,
                    };
                    let force = force_getter(obj_slice, other_slice, epsilon);
                    // update velocities
                    obj_slice[3] += force[0] / obj_slice[0] * dt;
                    obj_slice[4] += force[1] / obj_slice[0] * dt;

                    // obj_slice[3] *= 0.99999;
                    // obj_slice[4] *= 0.99999;
                    // obj_slice[3] *= 0.98;
                    // obj_slice[4] *= 0.98;
                }
            }
        }

        // for field in ...
        //     for interaction in field_interactions.iter() {
        //         // TODO even do this here? maybe other new integrator..?
        //     }
        // }

        // update positions
        obj_slice[1] += obj_slice[3] * dt;
        obj_slice[2] += obj_slice[4] * dt;
    }
}
