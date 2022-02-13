
use crate::state::State;
use crate::state::object::variant::ObjVariant;
use crate::state::object::family::ObjFamily;
use crate::interaction::object::object::Interaction as ObjInteraction;
use crate::interaction::object::field::Interaction as FieldInteraction;

use crate::interaction::object::object::forces;

use crate::config::EngineConfig;


pub fn step(
    iter_idx: usize,
    family: &mut ObjFamily,
    states: &Vec<State>,
    config: &EngineConfig,
) {

    let dt = config.dt;
    let epsilon = 0.05; // TODO: get from obj family? (& saved externally?)

    let obj_variant = &config.obj_families[family.id].obj_variant;
    if matches!(obj_variant, ObjVariant::Static) { return () }

    // get length of slice representing object in state vec
    let obj_length = &config.obj_families[family.id].obj_attributes.len();

    for obj_idx in 0..config.obj_families[family.id].family_size { 
        let obj_slice = &mut family.objects[obj_idx*obj_length..(obj_idx+1)*obj_length];

        for other_family in &states[iter_idx].obj_families {

            let other_variant = &config.obj_families[other_family.id].obj_variant;
            if matches!(other_variant, ObjVariant::Particle) { continue }

            // TODO get relevant neighbor: tree / sectors ?

            // get length of slice representing other object in state vec
            let other_length = &config.obj_families[other_family.id].obj_attributes.len();
           
            for other_idx in 0..config.obj_families[other_family.id].family_size { // ? TODO 0->obj_idx, update both bodies!
                // no self-interaction
                if family.id == other_family.id { if obj_idx == other_idx { continue } }
                // get slice representing other object in state vec
                let other_slice = &other_family.objects[
                    other_idx*other_length..(other_idx+1)*other_length
                ];
                
                // TODO check if both fams have this interaction
                let obj_interactions = &config.obj_families[family.id].obj_interactions;
                for interaction in obj_interactions.iter() {
                    // TODO handle forces different from other interactions
                    let force_getter = match interaction {
                        ObjInteraction::ForceNewtonianGravity => forces::newtonian_gravity::force,
                        ObjInteraction::ForceCoulomb => forces::coulomb::force,
                        ObjInteraction::ForceLennardJones => forces::lennard_jones::force,
                    };
                    let force = force_getter( obj_slice, other_slice, epsilon );
                    obj_slice[3] += force[0] / obj_slice[0] * dt;
                    obj_slice[4] += force[1] / obj_slice[0] * dt;
                }
            }
        }

        // for field in ...
        //     for interaction in field_interactions.iter() {
        //         // TODO even do this here? maybe other new integrator..?
        //     }
        // }

        // update positions from velocity
        obj_slice[1] += obj_slice[3] * dt;
        obj_slice[2] += obj_slice[4] * dt;
    }
}

