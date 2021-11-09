
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
    sim_id: String,
    canvas: Canvas,
}
impl Renderer {
    pub fn new(sim_id: String) -> Self {
        let canvas = Canvas::new("canvas_main", false);
        Renderer { canvas, sim_id }
    }
    pub fn init(&mut self) {
        // match self.sim_id.as_str() {
        //     "ising" => self.ising_init(),
        //     _ => ()
        // }
    }
    pub fn draw(&mut self, N: usize, state: &Vec<f64>) {
        
        self.canvas.clear();

        match self.sim_id.as_str() {
            "ising" => self.ising_draw(state, N),
            _ => ()
        }
    }
    // pub fn ising_init(&mut self) {

    // }
    pub fn ising_draw(&mut self, state: &Vec<f64>, N: usize) {

        self.canvas.set_fill_style("white");
        for i in 0..N {
            for j in 0..N {
                let N = N as f64;
                let x = i as f64 / N;
                let y = j as f64 / N;
                let z = 0.8;
                let center = (
                    x + (1.-z) / (2.*N), 
                    y + (1.-z) / (2.*N)
                );
                let width = 1. / N * z;
                let height = 1. / N * z;
                if state[i*N as usize+j] == 1. {
                    self.canvas.fill_rect(center, width, height)
                }
            }
        }

            // // let m = body.mass;
            // let r = 1.; //  2. * m; // TODO
            // let color = get_body_color(body.id);
            // let x = 0.; // canvas_width / 2.;
            // let y = 0.; // canvas_height / 2.;
            // let color = "purple";

            // self.canvas.set_stroke_style(&color);
            // self.canvas.set_fill_style(&color);
            // self.canvas.draw_circle((x, y), r, false)
    }
}
