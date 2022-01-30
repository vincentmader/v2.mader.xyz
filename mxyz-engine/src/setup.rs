
use crate::integrator::field::FieldIntegrator;
use crate::integrator::object::ObjectIntegrator;
use crate::boundary::object::ObjectBoundary;
// use crate::boundary::field::FieldBoundary;   // TODO



pub struct EngineSetup {

    pub field_integrators: Vec<FieldIntegrator>,   // one for each field
    pub object_integrators: Vec<ObjectIntegrator>, // one for each object family
    // pub field_boundaries: Vec<Vec<FieldBoundary>>,         // TODO
    pub object_boundaries: Vec<ObjectBoundary>,
    // pub object_interactions: Vec<ObjectInteractionSetup>;  // TODO
    // pub field_interactions: Vec<FieldInteractionSetup>;    // TODO

}
impl EngineSetup {

    pub fn new() -> Self {
        EngineSetup {
            field_integrators:  Vec::new(),  // TODO make Vec<Vec<FieldIntegrator>> ?
            object_integrators: Vec::new(), 
            object_boundaries:  Vec::new(),
        }
    }
}

