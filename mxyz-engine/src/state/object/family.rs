
use crate::state::object::variant::ObjVariant;
use crate::state::object::attribute::ObjAttribute;
use crate::config::obj_family::ObjFamilyEngineConfig;


#[derive(Clone)]
pub struct ObjFamily {

    pub id: usize,
    pub objects: Vec<f64>,

    pub obj_length: usize,

}
impl ObjFamily {

    pub fn new(
        id: usize,
    ) -> Self {

        let obj_length = 5; // attributes.len();
        let objects = Vec::new();
        // let nr_of_objects = objects.len() / obj_length;

        ObjFamily {
            id,
            objects,
            obj_length,
        }

    }

    pub fn add_object(
        &mut self, 
        obj: &Vec<f64>, 
        fam_conf: &mut ObjFamilyEngineConfig
    ) {
        self.objects.extend_from_slice(obj);  // TODO will lead to problems at live-add (eg. tails)
        fam_conf.family_size += 1;
    }
}

