
use crate::integrators::FieldIntegrator as Integrator;
use crate::interactions::FieldInteraction as Interaction;
mod field_variant;
pub use field_variant::FieldVariant;


#[derive(Clone)]
pub struct Field {
    pub id: usize,
    pub field_variant: FieldVariant,
    pub integrator: Integrator,
    pub interactions: Vec<Interaction>,
    pub dimensions: (usize, usize),   // TODO 3D ?
    pub cells: Vec<Vec<f64>>,         // TODO make Vec<bool> ?
}
impl Field {
    pub fn new(
        id: usize, 
        field_variant: FieldVariant,
        integrator: Integrator,
        interactions: Vec<Interaction>,
        dimensions: (usize, usize),
        cells: Vec<Vec<f64>>,
    ) -> Self {
        Field {
            id,
            field_variant,
            integrator,
            interactions,
            dimensions,
            cells,
        }
    }
}

