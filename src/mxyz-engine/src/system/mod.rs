
pub mod field;
pub mod obj_family;


pub enum System {
    Field(field::Field),
    ObjectFamily(obj_family::ObjFamily),
}



// pub enum StateVector {
//     Int(Vec<i32>),
//     Bool(Vec<bool>),
//     Float(Vec<f64>),
// //     Vec2Df(Vec<[f64; 2]>),
// //     Vec3Df(Vec<[f64; 3]>),
// }

// pub struct StateVector <T> {
//     cells:    Vec<T>,
// }
// impl <T> StateVector <T> {
//     pub fn new() -> Self {
//         StateVector {
//             cells:  Vec::new();
//         }
//     }
// }

