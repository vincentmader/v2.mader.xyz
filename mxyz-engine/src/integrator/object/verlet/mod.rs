
use crate::state::State;
use crate::state::object::ObjectFamily;
use crate::interaction::object::object::Interaction as ObjectInteraction;
use crate::interaction::object::field::Interaction as FieldInteraction;
use crate::config::EngineConfig;


pub fn step(
    iteration_idx: usize,
    family: &mut ObjectFamily,
    states: &Vec<State>,
    field_interactions: &Vec<FieldInteraction>,
    object_interactions: &Vec<ObjectInteraction>,
    dt: f64,
    config: &EngineConfig,
) {

}


