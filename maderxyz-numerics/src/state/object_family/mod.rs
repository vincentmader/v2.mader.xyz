
use crate::integrators::Integrator;
use crate::interactions::ObjectInteraction;


#[derive(Clone)]
pub struct ObjectFamily {

    pub id: usize,
    pub object_type: ObjectType,
    pub objects: Vec<Vec<f64>>,
    pub interactions: Vec<ObjectInteraction>,
    pub integrator: Integrator,
    pub dt: f64,
    pub epsilon: f64,
    pub tail_length: usize,
    pub attributes: Vec<ObjectAttribute>
    // pub object_colors: Vec<String>,

}
impl ObjectFamily {

    pub fn new(
        id: usize,
        object_type: ObjectType,
        objects: Vec<Vec<f64>>,
        interactions: Vec<ObjectInteraction>,
        integrator: Integrator,
        dt: f64,
        epsilon: f64,
        tail_length: usize,
    ) -> Self {

        let attributes: Vec<ObjectAttribute> = Vec::new();
        // let object_colors = Vec::from([String::from("white")]);

        ObjectFamily { 
            id, object_type, objects, 
            interactions, integrator, dt, epsilon, 
            tail_length, 
            attributes,
            // object_colors,
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
