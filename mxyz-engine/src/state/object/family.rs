
use crate::state::object::variant::ObjVariant;
use crate::state::object::attribute::ObjAttribute;
use crate::config::obj_family::ObjFamilyEngineConfig;


#[derive(Clone)]
pub struct ObjFamily {

    pub id: usize,

    // pub variant: ObjVariant,
    // pub attributes: Vec<ObjAttribute>,

    pub objects: Vec<f64>,
    // pub states: Vec<Vec<f64>>,

    // pub family_size: usize,
    // pub obj_length: usize,

    // pub config: &ObjFamilyEngineConfig,

}
impl ObjFamily {

    pub fn new(
        id: usize,
        // variant: ObjVariant,
        // attributes: Vec<ObjAttribute>,
        // objects: Vec<f64>,
    ) -> Self {

        // let obj_length = 5; // attributes.len();
        let objects = Vec::new();

        // let family_size = objects.len() / 5;  // TODO
        // let states = Vec::from([objects]);

        ObjFamily {
            id,
            // variant,
            // attributes,
            objects,
            // states,
            // family_size,
            // obj_length,
        }

    }

    pub fn add_object(&mut self, obj: &Vec<f64>, config: &mut ObjFamilyEngineConfig) {
        // let iter_idx = self.states.len() - 1;
        // self.states[iter_idx].extend_from_slice(obj);  // TODO will lead to problems at live-add (eg. tails)
        self.objects.extend_from_slice(obj);  // TODO will lead to problems at live-add (eg. tails)
        config.family_size += 1;
    }
}

