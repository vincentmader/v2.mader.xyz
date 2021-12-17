
use crate::state::object::ObjectFamily;

pub mod variant;
use variant::ObjectBoundaryVariant;

mod periodic;
mod collision;


pub struct ObjectBoundary {

    variant: ObjectBoundaryVariant,

}
impl ObjectBoundary {

    pub fn new(
        variant: ObjectBoundaryVariant,
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
            ObjectBoundaryVariant::Periodic => periodic::apply,
            ObjectBoundaryVariant::WallCollisionElastic => collision::wall::elastic::apply,
            ObjectBoundaryVariant::WallCollisionInelastic => collision::wall::inelastic::apply,
        };

        applier(object_family);

    }

}

