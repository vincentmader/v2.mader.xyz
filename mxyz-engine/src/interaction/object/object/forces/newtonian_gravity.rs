
pub fn force(
    object: &[f64], 
    other: &[f64], 
    dt: f64, 
    epsilon: f64
) -> Vec<f64> {
// }
// pub fn apply(
//     object: &mut [f64], 
//     other: &[f64], 
//     dt: f64, 
//     epsilon: f64
// ) {

    // atributes of object
    let m = object[0];
    let x = object[1];
    let y = object[2];
    // atributes of other object
    let M = other[0];
    let X = other[1];
    let Y = other[2];
    
    // distance & force
    let dx = X-x;
    let dy = Y-y;
    let r = ( dx.powf(2.) + dy.powf(2.) ).sqrt();
        // define: G -> 1
    let force = (m*M) * r/( r.powf(2.) + epsilon.powf(2.) ).powf(1.5);
    let force_x = force * dx/r;
    let force_y = force * dy/r;

    // let acc_x = force / m * dx/r;
    // let acc_y = force / m * dy/r;

    // object[3] += acc_x * dt;
    // object[4] += acc_y * dt;

    Vec::from([force_x, force_y])

}

