
pub struct State {

    bodies: Vec<f64>,
    particles: Vec<f64>,
    fields: Vec<f64>,
    cells: Vec<f64>,
    spins: Vec<bool>,
    // SubState? 

}
impl State {

    pub fn new(page_id: &str) -> Self {
        let bodies: Vec<f64> = Vec::new();
        let particles: Vec<f64> = Vec::new();
        let fields: Vec<f64> = Vec::new();
        let cells: Vec<f64> = Vec::new();
        let spins: Vec<bool> = Vec::new(); 
        State { bodies, particles, fields, cells, spins }
    }
}

