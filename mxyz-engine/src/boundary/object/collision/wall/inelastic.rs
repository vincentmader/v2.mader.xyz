
use crate::state::object::ObjectFamily;


pub fn apply(
    object_family: &mut ObjectFamily
) {

    // let iteration_idx = object_family.states.len();
    let tamper = 0.5;  // TODO  make configurable

    const X_MIN: f64 = -1.;  // TODO make bounds configurable
    const X_MAX: f64 = 1.;
    const Y_MIN: f64 = -1.;
    const Y_MAX: f64 = 1.;

    let obj_length = object_family.object_length;
    let objects = &mut object_family.objects;

    for obj_idx in 0..object_family.nr_of_objects {

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
