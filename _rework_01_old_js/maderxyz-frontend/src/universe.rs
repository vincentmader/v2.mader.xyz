use wasm_bindgen::prelude::*;

// mod cell;
// use cell::Cell;


#[wasm_bindgen]
pub struct Universe {
    namez: String,
    dimensions: (u16, u16),
    // cells: Vec<f64>,
}
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        self.namez = String::from("beeloooooooooo");
    }
    pub fn new() -> Universe {
        let namez = String::from("beele");
        let dimensions = (100, 100);
        // let cells = Vec::new();
        // for x_idx in 0..dimensions.0 {
        //     for y_idx in 0..dimensions.1 {
        //         // let cell = Cell::new();
        //         // cells.push(cell)
        //         println!("ay!")
        //     }
        // }
        Universe {
            namez, 
            dimensions, 
            // cells
        }
    }
    pub fn name(&self) -> String {
        String::from(&self.namez)
    }
}
