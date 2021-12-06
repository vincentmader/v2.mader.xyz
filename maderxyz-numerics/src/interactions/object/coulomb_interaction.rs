
pub fn force(

    object: &mut Vec<f64>, 
    other: &Vec<f64>, 
    epsilon: f64  // TODO handle

) -> (f64, f64) {

    const K: f64 = 1.;  

    let q1 = object[5];
    let x1 = object[1];
    let y1 = object[2];
    let q2 = other[5];
    let x2 = other[1];
    let y2 = other[2];
    let delta_x = x2 - x1;
    let delta_y = y2 - y1;
    let r = ( delta_x.powf(2.) + delta_y.powf(2.) ).sqrt();

    let force = -K * (q1*q2) / ( r.powf(2.) + epsilon.powf(2.) );
    let force_x = (delta_x / r) * force;
    let force_y = (delta_y / r) * force;
    (force_x, force_y)
}

