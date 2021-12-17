
use crate::state::object::ObjectVariant;
use crate::state::object::ObjectAttribute;


#[derive(Clone)]
pub struct ObjectFamily {
    pub id: usize,
    pub variant: ObjectVariant,
    pub attributes: Vec<ObjectAttribute>,
    pub objects: Vec<f64>,
    pub nr_of_objects: usize,
    pub object_length: usize,
}
impl ObjectFamily {

    pub fn new(
        id: usize,
        variant: ObjectVariant,
        attributes: Vec<ObjectAttribute>,
        objects: Vec<f64>,
    ) -> Self {

        let object_length = attributes.len();
        let nr_of_objects = objects.len() / object_length;

        ObjectFamily {
            id,
            variant,
            attributes,
            objects,
            nr_of_objects,
            object_length,
        }

    }

    pub fn add_object(&mut self, obj: &Vec<f64>) {
        self.objects.extend_from_slice(obj);
        self.nr_of_objects += 1;
    }

}

