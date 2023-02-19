
// use crate::state::object::family::ObjFamily;

pub mod variant;
// use variant::BoundaryVariant;

pub mod none;
pub mod periodic;
pub mod collision;



// pub struct ObjBoundary {

//     pub variant: BoundaryVariant,

// }
// impl ObjBoundary {

//     pub fn new(
//         variant: BoundaryVariant,
//     ) -> Self {

//         ObjBoundary {
//             variant,
//         }

//     }

//     pub fn apply(
//         &mut self, 
//         obj_family: &mut ObjFamily
//     ) {

//         let applier = match self.variant {
//             BoundaryVariant::None => none::apply,
//             BoundaryVariant::Periodic => periodic::apply,
//             BoundaryVariant::WallCollisionElastic => collision::wall::elastic::apply,
//             BoundaryVariant::WallCollisionInelastic => collision::wall::inelastic::apply,
//         };

//         applier(obj_family);

//     }

// }

