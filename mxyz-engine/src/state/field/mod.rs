
pub mod variant;
pub use variant::FieldVariant;
use crate::integrator::field::variant::FieldIntegratorVariant;


#[derive(Clone)]
pub struct Field {

    pub id: usize,
    // pub variant: FieldVariant,  // TODO move to conf
    pub entries: Vec<f64>,
    // pub config: FieldEngineConfig,

}

impl Field {
    pub fn new(

        id: usize,
        // variant: FieldVariant,
        entries: Vec<f64>,

    ) -> Self {
        Field { id, entries }
    }
}

