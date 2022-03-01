
use crate::state::field::Field;
use crate::state::object::ObjFamily;


pub enum System {
    Field(Field),
    ObjectFamily(ObjFamily),
}

