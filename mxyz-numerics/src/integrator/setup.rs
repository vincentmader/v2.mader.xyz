
use crate::integrator::field::FieldIntegrator;
use crate::integrator::object::ObjectIntegrator;
// use crate::boundary::object::ObjectBoundary;
use crate::boundary::object::ObjectBoundary;


pub struct IntegratorSetup {
    pub field: Vec<Vec<FieldIntegrator>>,
    pub object: Vec<Vec<ObjectIntegrator>>,
    // pub field_boundaries: Vec<Vec<FieldBoundary>>,
    pub object_boundaries: Vec<Vec<ObjectBoundary>>,
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

