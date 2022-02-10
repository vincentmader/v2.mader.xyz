

pub fn force(
    object: &[f64], 
    other: &[f64], 
    // dt: f64, 
    epsilon: f64
) -> Vec<f64> {

    // atributes of object
    let x_1 = object[1];
    let y_1 = object[2];
    let q_1 = object[0];

    // atributes of other object
    let x_2 = other[1];
    let y_2 = other[2];
    let q_2 = other[0];
    
    // distance & force
    let dx = x_2 - x_1;
    let dy = y_2 - y_1;
    let r = ( dx.powf(2.) + dy.powf(2.) ).sqrt();
        // define: k -> 1
    let force = -(q_1*q_2) * r/( r.powf(2.) + epsilon.powf(2.) ).powf(1.5);
    let force_x = force * dx/r;
    let force_y = force * dy/r;

    Vec::from([force_x, force_y])
}

