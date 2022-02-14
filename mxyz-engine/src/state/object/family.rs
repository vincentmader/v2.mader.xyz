
use crate::state::object::variant::ObjVariant;
use crate::state::object::attribute::ObjAttribute;
use crate::config::obj_family::ObjFamilyEngineConfig;


#[derive(Clone)]
pub struct ObjFamily {

    pub id: usize,
    pub objects: Vec<f64>,

}

impl ObjFamily {

    pub fn new(id: usize) -> Self {
        ObjFamily {
            id,
            objects: Vec::new(),
        }
    }

    pub fn add_object(
        &mut self, 
        obj: &Vec<f64>, 
        config: &mut ObjFamilyEngineConfig
    ) {
        self.objects.extend_from_slice(obj);  // TODO might lead to problems at live-add (eg. tails)
        config.family_size += 1;
    }
}

