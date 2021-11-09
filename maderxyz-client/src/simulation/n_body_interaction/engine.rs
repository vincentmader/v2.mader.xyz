use rand::Rng;


pub struct Engine {
    pub state: Vec<f64>,
    page_id: String,
}
impl Engine {
    pub fn new(page_id: String) -> Self {
        let state = vec![];
        Engine {
            state, page_id,
        }
    }
    pub fn init(&mut self) {
        let mut rng = rand::thread_rng();
    }
    pub fn step(&mut self) {
        let mut rng = rand::thread_rng();
    }
}
