
use crate::interactions::field::Interaction as FieldInteraction;
mod field_variant;
pub use field_variant::FieldVariant;
mod neighborhood_variant;
pub use neighborhood_variant::NeighborhoodVariant;
pub mod neighborhood_getter;


#[derive(Clone)]
pub struct Field {
    pub id: usize,
    pub field_variant: FieldVariant,
    pub interactions: Vec<FieldInteraction>,
    pub dimensions: (usize, usize),   // TODO 3D ?
    pub cells: Vec<Vec<f64>>,         // TODO make Vec<bool> ?
    pub neighborhood_variant: NeighborhoodVariant,
}
impl Field {
    pub fn new(
        id: usize, 
        field_variant: FieldVariant,
        interactions: Vec<FieldInteraction>,
        dimensions: (usize, usize),
        cells: Vec<Vec<f64>>,
    ) -> Self {
        let neighborhood_variant = NeighborhoodVariant::Moore;
        Field {
            id,
            field_variant,
            interactions,
            dimensions,
            cells,
            neighborhood_variant,
        }
    }
}

