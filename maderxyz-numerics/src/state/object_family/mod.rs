
use crate::interactions::object::Interaction as ObjectInteraction;


#[derive(Clone)]
pub enum TailVariant {
    Default,
    Area,
}


#[derive(Clone)]
pub struct ObjectFamily {

    pub id: usize,
    pub object_type: ObjectType,
    pub objects: Vec<Vec<f64>>,
    pub interactions: Vec<ObjectInteraction>,
    pub dt: f64,
    pub epsilon: f64,
    pub tail_length: usize,
    pub attributes: Vec<ObjectAttribute>,  // TODO obj colors, radii, ...
    pub tail_variant: TailVariant,

}
impl ObjectFamily {

    pub fn new(
        id: usize,
        object_type: ObjectType,
        objects: Vec<Vec<f64>>,
        interactions: Vec<ObjectInteraction>,
        dt: f64,
    ) -> Self {

        let attributes: Vec<ObjectAttribute> = Vec::new();
        let epsilon = 0.;
        let tail_length = 0;
        let tail_variant = TailVariant::Area;

        ObjectFamily { 
            id, object_type, objects, interactions, 
            dt, epsilon, tail_length, attributes, tail_variant,
        }
    }

}

#[derive(Clone, PartialEq)]
pub enum ObjectAttribute {
    Charge,
}

#[derive(Clone)]
pub enum ObjectType {
    Particle, Body, Static,
}