
pub mod relevant_cells;
pub mod variant;


#[derive(Clone)]
pub struct Field {

    pub id: usize,
    pub entries: Vec<f64>,

}

impl Field {

    pub fn new(id: usize) -> Self {
        Field { 
            id, 
            entries: Vec::new(),
        }
    }
}



// enum FieldCell {
//     Bool(bool),
//     Float(f64),
//     Vector(Vec<Vector>),
// }

// pub struct Vector {

// }
