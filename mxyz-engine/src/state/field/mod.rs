
pub mod variant;
pub use variant::FieldVariant;


#[derive(Clone)]
pub struct Field {

    pub id: usize,
    pub variant: FieldVariant,
    pub entries: Vec<f64>,

}

impl Field {
    pub fn new(

        id: usize,
        variant: FieldVariant,
        entries: Vec<f64>,

    ) -> Self {
        Field { id, variant, entries }
    }
}

