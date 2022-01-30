
use crate::state::field::Field;

pub mod variant;
use variant::BoundaryVariant;

mod none;
mod periodic;


pub struct FieldBoundary {

    pub variant: BoundaryVariant,

}
impl FieldBoundary {

    pub fn new(
        variant: BoundaryVariant,
    ) -> Self {

        FieldBoundary {
            variant,
        }

    }

    pub fn apply(
        &mut self, 
        field: &mut Field
    ) {

        // let applier = match self.variant {
            // BoundaryVariant::None => none::apply,
            // BoundaryVariant::Periodic => periodic::apply,
            // BoundaryVariant::WallCollisionElastic => collision::wall::elastic::apply,
            // BoundaryVariant::WallCollisionInelastic => collision::wall::inelastic::apply,
        // };

        // applier(object_family);

    }

}

