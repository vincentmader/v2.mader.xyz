
use crate::state::State;
use crate::state::object::ObjectVariant;
use crate::state::object::ObjectFamily;
use crate::interaction::object::object::Interaction as ObjectInteraction;
use crate::interaction::object::field::Interaction as FieldInteraction;

use crate::interaction::object::object::forces;


pub fn step(
    iteration_idx: usize,
    family: &mut ObjectFamily,
    states: &Vec<State>,
    field_interactions: &Vec<FieldInteraction>,
    object_interactions: &Vec<ObjectInteraction>,
    dt: f64,
) {

    // TODO make changeable
    const epsilon: f64 = 0.05; // todo: get from obj family? (& saved externally?)

    if matches!(family.variant, ObjectVariant::Static) { return () }

    // get length of slice representing object in state vec
    let object_length = family.object_length;

    for obj_idx in 0..family.nr_of_objects { 
        let obj_slice = &mut family.objects[obj_idx*object_length..(obj_idx+1)*object_length];

        for other_family in &states[iteration_idx].object_families {
            if matches!(other_family.variant, ObjectVariant::Particle) { continue }

            // TODO get relevant neighbor: tree / sectors ?

            // get length of slice representing other object in state vec
            let other_length = other_family.object_length;
           
            for other_idx in 0..other_family.nr_of_objects { // ? TODO 0->obj_idx, update both bodies!
                // no self-interaction
                if (family.id, obj_idx) == (other_family.id, other_idx) { continue }
                // get slice represinting other object in state vec
                let other_slice = &other_family.objects[
                    other_idx*other_length..(other_idx+1)*other_length
                ];
                
                for interaction in object_interactions.iter() {

                    let force = match interaction {
                        ObjectInteraction::ForceNewtonianGravity => forces::newtonian_gravity::force,
                        ObjectInteraction::ForceCoulomb => forces::coulomb::force,
                        ObjectInteraction::ForceLennardJones => forces::lennard_jones::force,
                    };
                    let force = force( obj_slice, other_slice, dt, epsilon );
                    obj_slice[3] += force[0] / obj_slice[0] * dt;
                    obj_slice[4] += force[1] / obj_slice[0] * dt;

                }

                // for interaction in field_interactions.iter() {
                //     // TODO even do this here? maybe other new integrator..?
                // }

            }
        }

        // update positions from velocity
        obj_slice[1] += obj_slice[3] * dt;
        obj_slice[2] += obj_slice[4] * dt;

        // let foo = 0.3;
        // obj_slice[3] *= foo;
        // obj_slice[4] *= foo;

    }

}

