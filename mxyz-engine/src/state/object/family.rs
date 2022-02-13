
use crate::state::object::variant::ObjVariant;
use crate::state::object::attribute::ObjAttribute;
use crate::config::obj_family::ObjFamilyEngineConfig;


#[derive(Clone)]
pub struct ObjFamily {

    pub id: usize,
    pub objects: Vec<f64>,

}
impl ObjFamily {

    pub fn new(
        id: usize,
    ) -> Self {

        let objects = Vec::new();

        ObjFamily {
            id,
            objects,
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

