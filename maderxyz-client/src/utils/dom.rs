// use std::cell::RefCell;
// use std::cell::Cell;
// use std::rc::Rc;

// use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;


pub fn window() -> web_sys::Window {
    web_sys::window()
        .expect("no global `window` exists")
}

// pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
//     window()
//         .request_animation_frame(f.as_ref().unchecked_ref())
//         .expect("should register 'requestAnimationFrame' OK");
// }

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

// pub fn body() -> web_sys::HtmlElement {
//     document()
//         .body()
//         .expect("document should have a body")
// }

pub fn canvas(canvas_id: &str) -> web_sys::HtmlCanvasElement {
    document()
        .get_element_by_id(canvas_id)
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

pub fn ctx(canvas: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}


use wasm_bindgen::JsValue;

pub struct Canvas {
    element: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
    dimensions: (f64, f64),
    centered: bool,
}
impl Canvas {
    pub fn new(canvas_id: &str, centered: bool) -> Self {
        let element = canvas(canvas_id);
        let context = ctx(&element);
        let canvas_width = f64::from(element.width());
        let canvas_height = f64::from(element.height());
        let dimensions = (canvas_width, canvas_height);
        let centered = centered;
        Canvas {
            element, context, dimensions, centered
        }
    }
    pub fn clear(&mut self) {
        let w = self.dimensions.0;
        let h = self.dimensions.1;
        self.context.clear_rect(0., 0., w, h);
    }
    pub fn set_stroke_style(&mut self, color: &str) {
        self.context.set_stroke_style(&JsValue::from_str(&color));
    }
    pub fn set_fill_style(&mut self, color: &str) {
        self.context.set_fill_style(&JsValue::from_str(&color));
    }
    pub fn draw_line(
        &mut self, 
        mut from: (f64, f64), 
        mut to: (f64, f64),
    ) {
        // center
        if self.centered {
            from = (
                from.0 + 0.5,
                from.1 + 0.5,
            );
            to = (
                to.0 + 0.5,
                to.1 + 0.5,
            );
        }
        // re-scale
        from = (
            from.0 * self.dimensions.0,
            from.1 * self.dimensions.1,
        );
        to = (
            to.0 * self.dimensions.0,
            to.1 * self.dimensions.1,
        );

        // draw
        self.context.begin_path();
            self.context.move_to(from.0, from.1);
            self.context.line_to(to.0, to.1);
        self.context.stroke();
    }
    pub fn draw_circle(
        &mut self, 
        mut center: (f64, f64), 
        radius: f64,
        fill: bool
    ) {
        const TAU: f64 = 2.0 * std::f64::consts::PI;
        // center
        if self.centered {
            center = (
                center.0 + 0.5,
                center.1 + 0.5,
            );
        }
        // re-scale
        center = (
            center.0 * self.dimensions.0,
            center.1 * self.dimensions.1,
        );
        let radius = radius * 0.5 * self.dimensions.1;  // TODO: only works for square
        // draw
        self.context.begin_path();
        self.context.arc( center.0, center.1, radius, 0.0, TAU ).unwrap();
        self.context.stroke();
        if fill {
            self.context.fill();
        }
    }
    pub fn fill_rect(
        &mut self, 
        mut center: (f64, f64), 
        width: f64,
        height: f64,
    ) {
        // center
        if self.centered {
            center = (
                center.0 + 0.5,
                center.1 + 0.5,
            );
        }
        // re-scale
        center = (
            center.0 * self.dimensions.0,
            center.1 * self.dimensions.1,
        );
        let width = width * self.dimensions.0;
        let height = height * self.dimensions.1;
        // draw
        self.context.begin_path();
        self.context.fill_rect(
            center.0, center.1, width, height
        )
    }
}

pub fn console_log(x: &str) {
    let array = js_sys::Array::new();
    array.push(&x.into());
    web_sys::console::log(&array);
}

