
use crate::integrator::field::FieldIntegrator;
use crate::integrator::object::ObjectIntegrator;
// use crate::boundary::object::ObjectBoundary;
use crate::boundary::object::ObjectBoundary;


// pub struct ObjectInteractionSetup {
     
// }
// impl ObjectInteractionSetup {
//     pub fn new() -> Self {
//         ObjectInteractionSetup {

//         }
//     }
// }




pub struct IntegratorSetup {

    pub field: Vec<FieldIntegrator>,   // one for each field
    pub object: Vec<ObjectIntegrator>, // one for each object family

    // pub field_boundaries: Vec<Vec<FieldBoundary>>,
    pub object_boundaries: Vec<ObjectBoundary>,

    // pub object_interactions: Vec<ObjectInteractionSetup>;
    // pub field_interactions: Vec<FieldInteractionSetup>;

}
impl IntegratorSetup {
    pub fn new() -> Self {
        // let field: Vec<Vec<FieldIntegrator>> = Vec::new();
        // let object: Vec<Vec<ObjectIntegrator>> = Vec::new();
        // let object_boundaries: Vec<Vec<ObjectBoundaryVariant>> = Vec::new();
        let field = Vec::new();
        let object = Vec::new();
        let object_boundaries = Vec::new();
        IntegratorSetup {
            field, object, object_boundaries,
        }
    }
}

