
use crate::integrator::field::FieldIntegrator;


pub struct FieldEngineConfig {

    pub id: usize,
    // pub dimensions: Vec<usize>,
    // matrix for interactions (?)
    // pub initial_state: Vec<f64>,   // TODO: bool / int ?
    // pub integrator: FieldIntegrator,
    // pub boundary: FieldBoundary,

}
impl FieldEngineConfig {

    pub fn new(id: usize, dt: f64) -> Self {
        FieldEngineConfig {
            id,
            // dimensions: Vec<usize>,
            // integrator
        }
    }
}

