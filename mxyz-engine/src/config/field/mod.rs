#![allow(non_snake_case)]

use crate::integrator::field::variant::FieldIntegratorVariant;
// use crate::state::field::variant::FieldVariant;
use crate::interaction::field::field::FieldFieldInteraction;
use crate::interaction::field::object::FieldObjInteraction;
use crate::boundary::field::variant::FieldBoundaryVariant;
use crate::state::field::relevant_cells::FieldRelevantCells;


pub struct FieldEngineConfig {

    pub id:                 usize,

    pub dimensions:         Vec<usize>,
    // pub dimensionality:     FieldDimensionalityVariant,

    pub field_interactions: Vec<FieldFieldInteraction>,
    pub obj_interactions:   Vec<FieldObjInteraction>,
    pub integrator:         FieldIntegratorVariant,
    pub boundary:           FieldBoundaryVariant,

    pub relevant_cells:     FieldRelevantCells,
    // pub field_variant:      FieldVariant,
    // pub cell_type:          FieldCellType,  // TODO bool / int / float

}
impl FieldEngineConfig {

    pub fn new(
        id: usize, 
        // dt: f64,
    ) -> Self {

        // let DEFAULT_FIELD_VARIANT       = FieldVariant::Ising;
        let DEFAULT_DIMENSIONS          = Vec::new();
        // let DEFAULT_DIMENSIONALITY      = FieldDimensionalityVariant::Uninitialized;
        let DEFAULT_INTEGRATOR          = FieldIntegratorVariant::CellAuto;
        let DEFAULT_FIELD_INTERACTIONS  = Vec::new();
        let DEFAULT_OBJ_INTERACTIONS    = Vec::new();
        let DEFAULT_RELEVANT_CELLS      = FieldRelevantCells::Entire;
        let DEFAULT_BOUNDARY_VARIANT    = FieldBoundaryVariant::None;

        FieldEngineConfig {
            id,
            // field_variant:      DEFAULT_FIELD_VARIANT,
            dimensions:         DEFAULT_DIMENSIONS,
            field_interactions: DEFAULT_FIELD_INTERACTIONS,
            obj_interactions:   DEFAULT_OBJ_INTERACTIONS,
            integrator:         DEFAULT_INTEGRATOR,
            boundary:           DEFAULT_BOUNDARY_VARIANT,
            relevant_cells:     DEFAULT_RELEVANT_CELLS,
        }
    }

    // pub fn set_resolution(&mut self, resolution: Vec<usize>) {
    //     self.dimensionality = match resolution.len() {
    //         _ => FieldDimensionalityVariant::Uninitialized,
    //     };
    //     self.dimensions = resolution;
    // }
}



// pub enum FieldDimensionalityVariant {
//     Uninitialized,
//     OneDimensional,
//     TwoDimensional,
//     ThreeDimensional,
// }
