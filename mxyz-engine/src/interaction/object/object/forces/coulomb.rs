
pub fn force(
    object: &[f64], 
    other: &[f64], 
    dt: f64, 
    epsilon: f64
) -> Vec<f64> {

    // atributes of object
    let m = object[0];
    let x = object[1];
    let y = object[2];
    let q = object[5];
    // atributes of other object
    let X = other[1];
    let Y = other[2];
    let Q = other[5];
    
    // distance & force
    let dx = X-x;
    let dy = Y-y;
    let r = ( dx.powf(2.) + dy.powf(2.) ).sqrt();
        // define: k -> 1
    let force = -(q*Q) * r/( r.powf(2.) + epsilon.powf(2.) ).powf(1.5);
    let force_x = force * dx/r;
    let force_y = force * dy/r;

    Vec::from([force_x, force_y])
}

