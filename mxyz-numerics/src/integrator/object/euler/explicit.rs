
use crate::state::State;
use crate::state::object::ObjectVariant;
use crate::state::object::ObjectFamily;
use crate::interaction::object::object::Interaction as ObjectInteraction;
use crate::interaction::object::field::Interaction as FieldInteraction;

use crate::interaction::object::object::forces;


pub fn step(
    object_family: &mut ObjectFamily,
    states: &Vec<State>,
    field_interactions: &Vec<FieldInteraction>,
    object_interactions: &Vec<ObjectInteraction>,
    dt: f64,
    // neighborhood
) {

    const epsilon: f64 = 0.05; // todo: get from obj family? (& saved externally?)

    let previous_state = &states[ states.len()-1 ];
    let object_length = object_family.object_length;
    let objects = &mut object_family.objects;

    for obj_idx in 0..object_family.nr_of_objects { 
        let start_idx = obj_idx * object_length;   

        let obj_slice = &mut objects[ 
            start_idx..start_idx+object_length
        ];

        for other_family in &previous_state.object_families {
            if matches!(other_family.variant, ObjectVariant::Particle) { continue }
            // get length of slice representing object in state vec
            let other_length = other_family.object_length;
           
            for other_idx in 0..other_family.nr_of_objects { // ? TODO 0->obj_idx, update both bodies!
                // no self-interaction
                if (object_family.id, obj_idx) == (other_family.id, other_idx) { continue }

                let other_slice = &other_family.objects[ 
                    other_idx*object_length..other_idx*other_length+object_length
                ];
                
                for interaction in object_interactions.iter() {

                    let force = match interaction {
                        ObjectInteraction::ForceNewtonianGravity => forces::newtonian_gravity::force,
                        // ObjectInteraction::ForceCoulomb => forces::coulomb::force,
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


    }

}

