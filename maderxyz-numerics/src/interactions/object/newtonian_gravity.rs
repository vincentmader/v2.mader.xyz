
pub fn force(

    object: &mut Vec<f64>, 
    other: &Vec<f64>, 
    epsilon: f64,  // TODO handle differently?

) -> (f64, f64) {

    const G: f64 = 1.;  

    let m1 = object[0];
    let m2 = other[0];
    let delta_x = other[1] - object[1];
    let delta_y = other[2] - object[2];
    let r = ( delta_x.powf(2.) + delta_y.powf(2.) ).sqrt();

    let force = G * (m1*m2) / ( r.powf(2.) + epsilon.powf(2.) ); // TODO why not (r+e)^2
    let force_x = (delta_x / r) * force;
    let force_y = (delta_y / r) * force;
    (force_x, force_y)
}

