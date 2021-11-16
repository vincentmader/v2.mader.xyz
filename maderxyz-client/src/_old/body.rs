// // mod utils;
// use crate::utils::Vector2D;
// // use std::f64;

// use wasm_bindgen::prelude::*;

// use crate::utils::alert;


// // #[derive(Copy, Clone)]
// pub struct Body {
//     pub id: u32,
//     pub mass: f64,
//     pub location: Vector2D,
//     pub velocity: Vector2D,
//     pub acceleration: Vector2D,
// }
// impl Body {
//     pub fn new(
//         id: u32,
//         mass: f64, 
//         location: Vector2D, 
//         velocity: Vector2D
//     ) -> Body {
//         Body {
//             id,
//             mass, 
//             location, 
//             velocity, 
//             acceleration: Vector2D::new(0., 0.),
//         }
//     }

//     pub fn update_vel(&mut self, force: Vector2D) {
//         let dt = 1.;
//         self.velocity = self.velocity + force / self.mass * dt;
//             // Vector2D::new(100., 0.)
//         // alert(&format!("{}, {}", self.location.x, self.location.y));
//     }

//     pub fn update_loc(&mut self) {
//         let dt = 1.;
//         self.location = self.location + self.velocity * dt;
//         // alert(&format!("{}, {}", self.location.x, self.location.y));
//     }
// }
