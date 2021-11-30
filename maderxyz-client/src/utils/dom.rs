// use std::cell::RefCell;
// use std::cell::Cell;
// use std::rc::Rc;

// use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use gloo::{events::EventListener};


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
    // element: web_sys::HtmlCanvasElement,
    pub context: web_sys::CanvasRenderingContext2d,
    dimensions: (f64, f64),
    pub centered: bool,
    scale: f64,
    pub zoom: f64,
}
impl Canvas {
    pub fn new(canvas_id: &str, centered: bool) -> Self {
        let element = canvas(canvas_id);
        let context = ctx(&element);
        let scale   = 1.;
        let canvas_width  = scale * f64::from(element.width());
        let canvas_height = scale * f64::from(element.height());
        let dimensions = (canvas_width, canvas_height);
        let zoom = match centered { true => 0.5, _ => 1.}; // TODO make changeable
        Canvas {
            // element, 
            context, dimensions, centered, scale, zoom,
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
    pub fn rescale_vec(&self, vec: (f64, f64)) -> (f64, f64) {
        // re-scale to canvas dimensions [px]   // TODO include scale
        let mut vec = vec;
        vec.0 *= self.zoom * self.dimensions.0 / self.scale;
        vec.1 *= self.zoom * self.dimensions.1 / self.scale;
        // center
        if self.centered {
            vec.0 += 0.5*self.dimensions.0;
            vec.1 += 0.5*self.dimensions.1;
        } 
        vec
    }
    pub fn rescale_dist(&self, dist: f64) -> f64 {
        dist * self.dimensions.1 * self.zoom / self.scale  // only works for square
    }
    pub fn draw_line(
        &mut self, 
        mut from: (f64, f64), 
        mut to:   (f64, f64),
    ) {
        from = self.rescale_vec(from);
        to   = self.rescale_vec(to);
        // draw
        self.context.begin_path();
        self.context.move_to(from.0, from.1);
        self.context.line_to(to.0, to.1);
        self.context.stroke();
    }
    pub fn draw_triangle(
        &mut self, 
        mut first:  (f64, f64), 
        mut second: (f64, f64),
        mut third:  (f64, f64),
    ) {
        first = self.rescale_vec(first);
        second = self.rescale_vec(second);
        third = self.rescale_vec(third);
        // draw
        self.context.begin_path();
        self.context.move_to( first.0, first.1);
        self.context.line_to(second.0, second.1);
        self.context.line_to( third.0, third.1);
        self.context.stroke();
        self.context.fill()
    }
    pub fn draw_circle(
        &mut self, 
        center: (f64, f64), 
        radius:  f64,
        fill:    bool
    ) {
        const TAU: f64 = 2.0 * std::f64::consts::PI;

        let center = self.rescale_vec(center);
        let radius = self.rescale_dist(radius);

        // draw
        self.context.begin_path();
        self.context.arc(center.0, center.1, radius, 0.0, TAU).unwrap();
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
        let center = self.rescale_vec(center);
        let width = self.rescale_dist(width);
        let height = self.rescale_dist(height);
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


pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, 
    // we can call the `set_panic_hook` function at least 
    // once during initialization, and then we will get 
    // better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}


use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

pub fn add_button_to_menu<F>(
    text: &str,
    callback: F,
) where F: FnMut () {
    let document = document();
    let section = document.get_element_by_id("button_menu").unwrap();
	
	let button = document.create_element("button").unwrap()
		.dyn_into::<web_sys::HtmlButtonElement>().unwrap();
	button.set_text_content(Some(text));
	
	// let paragraph = document.create_element("p").unwrap()
	// 	.dyn_into::<web_sys::HtmlParagraphElement>().unwrap();                    
    // section.append_child(&paragraph).unwrap();

	let on_click = EventListener::new(
        &button, "click", 
        move |_event| {
		    web_sys::console::log_2(
                &"Hello World Gloo :%s".into(),
				&"WebAssemblyMan".into()
            );
		    // paragraph.set_text_content(Some("Gloo: Hello World"));
            // callback()
	    }
    );
	on_click.forget();     
	section.append_child(&button).unwrap();
}	
