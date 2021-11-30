
use crate::integrators::FieldIntegrator;
use crate::interactions::FieldInteraction;


#[derive(Clone)]
pub enum FieldType {
    Spin,
    Fluid,
    GameOfLife,
}

#[derive(Clone)]
pub struct Field {
    pub dimensions: (usize, usize), // 2d
    pub cells: Vec<Vec<f64>>,
    pub id: usize,
    pub field_type: FieldType,
    pub interactions: Vec<FieldInteraction>,
    pub integrator: FieldIntegrator,
}
impl Field {
    pub fn new(
        id: usize, 
        field_type: FieldType,
        interactions: Vec<FieldInteraction>,
        dimensions: (usize, usize),
        cells: Vec<Vec<f64>>,
        integrator: FieldIntegrator,
    ) -> Self {
        Field {
            id,
            field_type,
            interactions,
            dimensions,
            cells,
            integrator
        }
    }
}

