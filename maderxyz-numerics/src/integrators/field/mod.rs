
pub mod batch_wise;
pub mod entire;


#[derive(Clone)]
pub enum Integrator {
    // TODO 
    BatchWise,
    Entire,
}

