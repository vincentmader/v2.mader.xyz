
pub fn force(

    object: &mut Vec<f64>, 
    other: &Vec<f64>, 
    epsilon: f64,  // TODO handle differently?

) -> (f64, f64) {

    const EPSILON: f64 = 1.;  // different epsilon from above
    const K: f64 = 4. * EPSILON;
    const SIGMA: f64 = 1.;

    let delta_x = other[1] - object[1];
    let delta_y = other[2] - object[2];
    let r = ( delta_x.powf(2.) + delta_y.powf(2.) ).sqrt();

    let s = SIGMA / (r + epsilon);
    let force = -K * ( 12.*s.powf(11.) - 6.*s.powf(5.) );

    let force_x = (delta_x / r) * force;
    let force_y = (delta_y / r) * force;
    (force_x, force_y)
}

