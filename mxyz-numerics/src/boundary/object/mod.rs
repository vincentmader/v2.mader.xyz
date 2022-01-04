
use crate::state::object::ObjectFamily;

pub mod variant;
use variant::BoundaryVariant;

mod none;
mod periodic;
mod collision;


pub struct ObjectBoundary {

    pub variant: BoundaryVariant,

}
impl ObjectBoundary {

    pub fn new(
        variant: BoundaryVariant,
    ) -> Self {

        ObjectBoundary {
            variant,
        }

    }

    pub fn apply(
        &mut self, 
        object_family: &mut ObjectFamily
    ) {

        let applier = match self.variant {
            BoundaryVariant::None => none::apply,
            BoundaryVariant::Periodic => periodic::apply,
            BoundaryVariant::WallCollisionElastic => collision::wall::elastic::apply,
            BoundaryVariant::WallCollisionInelastic => collision::wall::inelastic::apply,
        };

        applier(object_family);

    }

}

