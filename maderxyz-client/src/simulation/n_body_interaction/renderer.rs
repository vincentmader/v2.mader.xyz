
use crate::utils::dom::Canvas;


// pub struct State {
//     data: Vec<f64>,
//     length
// }
// impl State {
//     pub fn new() -> Self {
//         State {  }
//     }
//     pub fn init(&mut self) {
//     }}
// pub struct State {
//     data:
//     length:
//     nr_of_elements:
//     entries_per_element
// }

pub struct Renderer {
    page_id: String,
    canvas: Canvas,
}
impl Renderer {
    pub fn new(page_id: String) -> Self {
        let canvas = Canvas::new("canvas_main", false);
        Renderer { canvas, page_id }
    }
    pub fn init(&mut self) {}
    pub fn draw(&mut self, state: &Vec<f64>) {
        // clear canvas
        self.canvas.clear();
        // draw on canvas
        match self.page_id.as_str() {
            _ => ()
        }
    }
}
