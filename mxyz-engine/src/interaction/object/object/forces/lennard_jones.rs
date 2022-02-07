
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
    // atributes of other object
    let X = other[1];
    let Y = other[2];
    
    // distance & force
    let dx = X-x;
    let dy = Y-y;
    let r = ( dx.powf(2.) + dy.powf(2.) ).sqrt();
        // define: sigma -> 1

    let sigma = 0.1;
    let eps = 1.;
    // V = 4 eps ( (s/r)^12 - (s/r)^6 )
    let force = 4. * eps * sigma * (
           6. * (r / sigma).powf(- 7.)
         -12. * (r / sigma).powf(-13.) 
    );
    let force_x = force * dx/r;
    let force_y = force * dy/r;

    Vec::from([force_x, force_y])
}

