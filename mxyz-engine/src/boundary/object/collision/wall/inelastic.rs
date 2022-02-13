
use crate::state::object::family::ObjFamily;
use crate::config::obj_family::ObjFamilyEngineConfig;


pub fn apply(
    obj_family: &mut ObjFamily,
    config: &ObjFamilyEngineConfig
) {

    // let iter_idx = obj_family.states.len();
    let tamper = 0.5;  // TODO  make configurable

    const X_MIN: f64 = -1.;  // TODO make bounds configurable
    const X_MAX: f64 = 1.;
    const Y_MIN: f64 = -1.;
    const Y_MAX: f64 = 1.;

    let obj_length = obj_family.obj_length;
    let objects = &mut obj_family.objects;

    for obj_idx in 0..config.family_size {

        let start_idx = obj_idx * obj_length;

        let obj_slice = &mut objects[start_idx..start_idx+obj_length];

        if obj_slice[1] < X_MIN { 
            obj_slice[1] = -1.;
            obj_slice[3] *= -tamper;
        } else if obj_slice[1] > X_MAX {
            obj_slice[1] = 1.;
            obj_slice[3] *= -tamper;
        }

        if obj_slice[2] < Y_MIN {  
            obj_slice[2] = -1.;
            obj_slice[4] *= -tamper;
        } else if obj_slice[2] > Y_MAX {
            obj_slice[2] = 1.;
            obj_slice[4] *= -tamper;
        }

    }

}
