
// use crate::state::ObjectFamily;
// use crate::interactions::object::Interaction as ObjectInteraction;
// use crate::interactions::object::InteractionVariant as ObjectInteractionVariant;
// use crate::interactions::object::newtonian_gravity;
// use crate::interactions::object::coulomb;
// use crate::interactions::object::lennard_jones;



pub fn step(
    // object_family: &mut ObjectFamily, 
    // other_family: &ObjectFamily, 
    // interaction: &ObjectInteraction,
) {

    // let epsilon = object_family.epsilon;
    // let dt = object_family.dt;
    // // choose interaction applier-method
    // let get_force = match interaction.interaction_variant {
    //     ObjectInteractionVariant::NewtonianGravity => newtonian_gravity::force, // TODO rename
    //     ObjectInteractionVariant::Coulomb => coulomb_interaction::force,
    //     // ObjectInteraction::WallCollision => coulomb_interaction::force,
    // };
    // // loop over body-pairs, apply interaction (only to one!)
    // for (object_idx, object) in object_family.objects.iter_mut().enumerate() {
    //     for (other_idx, other) in other_family.objects.iter().enumerate() {
    //         if (object_family.id, object_idx) == (other_family.id, other_idx) { continue }  // TODO do differently
    //         let force = get_force(object, other, epsilon);  // TODO handle epsilon
    //         // update velocities
    //         object[3] += force.0 / object[0] * dt;
    //         object[4] += force.1 / object[0] * dt;
    //     }
    //     // update positions  | TODO do this after interaction calc
    //     object[1] += object[3] * dt;
    //     object[2] += object[4] * dt;
    // }
}

