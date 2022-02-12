
use crate::state::object::variant::ObjVariant;
use crate::state::object::attribute::ObjAttribute;


#[derive(Clone)]
pub struct ObjFamily {

    pub id: usize,

    pub variant: ObjVariant,
    // pub attributes: Vec<ObjAttribute>,

    pub objects: Vec<f64>,
    // pub states: Vec<Vec<f64>>,

    pub nr_of_objects: usize,
    pub obj_length: usize,

}
impl ObjFamily {

    pub fn new(
        id: usize,
        variant: ObjVariant,
        // attributes: Vec<ObjAttribute>,
        // objects: Vec<f64>,
    ) -> Self {

        let obj_length = 5; // attributes.len();
        let objects = Vec::new();
        let nr_of_objects = objects.len() / obj_length;
        // let states = Vec::from([objects]);

        ObjFamily {
            id,
            variant,
            // attributes,
            objects,
            // states,
            nr_of_objects,
            obj_length,
        }

    }

    pub fn add_object(&mut self, obj: &Vec<f64>) {
        // let iter_idx = self.states.len() - 1;
        // self.states[iter_idx].extend_from_slice(obj);  // TODO will lead to problems at live-add (eg. tails)
        self.objects.extend_from_slice(obj);  // TODO will lead to problems at live-add (eg. tails)
        self.nr_of_objects += 1;
    }
}

