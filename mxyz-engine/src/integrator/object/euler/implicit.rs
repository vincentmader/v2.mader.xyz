
use crate::state::State;
use crate::state::object::ObjectFamily;
use crate::interaction::object::object::Interaction as ObjectInteraction;
use crate::interaction::object::field::Interaction as FieldInteraction;


pub fn step(
    family_idx: usize,
    state: &mut State,
    field_interactions: &Vec<FieldInteraction>,
    object_interactions: &Vec<ObjectInteraction>,
    dt: f64,
) {

}

