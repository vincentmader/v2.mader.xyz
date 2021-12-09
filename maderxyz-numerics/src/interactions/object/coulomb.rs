
pub fn force(

    object: &mut Vec<f64>, 
    other: &Vec<f64>, 
    epsilon: f64  // TODO handle

) -> (f64, f64) {

    const K: f64 = 1.;  

    let q1 = object[5];
    let q2 = other[5];
    let delta_x = other[1] - object[1];
    let delta_y = other[2] - object[2];
    let r = ( delta_x.powf(2.) + delta_y.powf(2.) ).sqrt();

    let force = -K * (q1*q2) / ( r.powf(2.) + epsilon.powf(2.) );
    let force_x = (delta_x / r) * force;
    let force_y = (delta_y / r) * force;
    (force_x, force_y)
}

