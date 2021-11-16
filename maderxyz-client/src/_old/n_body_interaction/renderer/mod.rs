
use crate::utils::dom::Canvas;

use super::engine::body::Body;


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
        let canvas = Canvas::new(
            "canvas_main", 
            true  // origin is at center of canvas
        );
        // match page_id.as_str() {
        //     "3body_fig8" => ()
        //     _ => ()
        // }
        Renderer { canvas, page_id }
    }
    pub fn init(&mut self) {}
    pub fn draw(&mut self, state: &Vec<Body>) {

        // clear canvas
        self.canvas.clear();

        // drawing setup
        let color = "cyan";
        self.canvas.set_fill_style(&color);

        // draw on canvas
        for body in state.iter() {
            let center = (body.position.x, body.position.y);
            let radius = 1./300.;

            self.canvas.draw_circle(center, radius, true);
        }
    }
}



//use wasm_bindgen::JsValue;

//use super::universe::Universe;
//use crate::utils::dom::canvas;
//use crate::utils::dom::ctx;


//pub struct Renderer {
//    cnv: web_sys::HtmlCanvasElement,
//    ctx: web_sys::CanvasRenderingContext2d,
//}
//impl Renderer {
//    pub fn new(
//    ) -> Self {
//        let cnv = canvas("canvas_main");
//        let ctx = ctx(&cnv);
//        Renderer {
//            cnv, 
//            ctx, 
//        }
//    }
//    pub fn init(&self) {

//    }
//    pub fn display(&self, universe: &Universe) {
//        const PI: f64 = std::f64::consts::PI;
//        const TAU: f64 = 2.0 * PI;
//        // 
//        let canvas_width = f64::from(self.cnv.width());
//        let canvas_height = f64::from(self.cnv.height());
//        // clear screen 
//        self.ctx.clear_rect(0., 0., canvas_width, canvas_height);
//        // loop over bodies and draw
//        for body in &universe.bodies {
//            let x = (body.position.x + 2.)/4. * canvas_width;
//            let y = (body.position.y + 2.)/4. * canvas_height;
//            let m = body.mass;
//            let r = 1.; //  2. * m; // TODO
//            let color = get_body_color(body.id);
//            self.ctx.set_stroke_style(&JsValue::from_str(&color));
//            self.ctx.set_fill_style(&JsValue::from_str(&color));
//            self.ctx.begin_path();
//                self.ctx.arc(x, y, r, 0.0, TAU).unwrap();
//                self.ctx.stroke();
//            self.ctx.fill();
//        }
//        // let x: u8 = 1;  // TODO: move to console_log fn
//        // let array = js_sys::Array::new();
//        // array.push(&x.into());
//        // web_sys::console::log(&array);
//    }
//}




//use std::collections::HashMap;

//pub fn get_body_color(body_id: u32) -> &'static str {
//    let color_map = HashMap::from([
//        (0, "white"),
//        // (1, "white"),
//        // (2, "white"),
//        (1, "red"),
//        // (2, "red"),
//        (2, "blue"),
//        // (4, "blue"),
//        // (4, "green"),
//        // (4, "green"),
//        // (4, "green"),
//    ]);
//    let color = match color_map.get(&body_id) {
//        Some(col) => col,
//        None => "white",
//    };
//    color
//}




//        // let x0 = canvas_width / 2.0;
//        // let y0 = canvas_height / 2.0;
//        // let r = f64::from(self.step_idx) / 10.0;
//        // self.ctx.begin_path();
//        // self.ctx.set_fill_style(&JsValue::from_str("purple"));
//        // self.ctx.arc(x0, y0, r, 0.0, TAU).unwrap();
//        // self.ctx.fill();
//        // self.ctx.begin_path();
//        // self.ctx.set_fill_style(&JsValue::from_str("black"));
//        // self.ctx.arc(x0, y0, 0.8*r, 0.0, TAU).unwrap();
//        // self.ctx.fill();
