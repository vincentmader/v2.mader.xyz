
pub mod variant;
pub use variant::FieldVariant;


#[derive(Clone)]
pub struct Field {

    pub id: usize,
    pub variant: FieldVariant,
    pub dimensions: (usize, usize, usize),
    pub entries: Vec<f64>,

}

impl Field {
    pub fn new(

        id: usize,
        variant: FieldVariant,
        dimensions: (usize, usize, usize),
        entries: Vec<f64>,

    ) -> Self {
        Field { id, variant, dimensions, entries }
    }
}

