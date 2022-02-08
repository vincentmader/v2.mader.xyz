
pub enum ObjectColorMode {
    // None,
    Default,
    Mass,
    Charge,
    Distance,
    Speed,
    HSLPosition,
    HSLVelocity,
}


// TODO only return rgb values, apply alpha later! (from tail_idx)

pub fn get_hsl_from_vec(vec: [f64; 2], alpha: f64) -> String {

    const TAU: f64 = 2. * 3.14159265358;
    let phi = vec[1].atan2(vec[0]) / TAU * 360.;
    let (h, s, l) = (phi, 100, 50);
    format!("hsla({}, {}%, {}%, {})", h, s, l, alpha)

}

pub fn get_object_color_from_velocity_angle(obj: &Vec<f64>, alpha: f64) -> String {

    get_hsl_from_vec([obj[3], obj[4]], alpha)

}

pub fn get_object_color_from_position_angle(obj: &Vec<f64>, alpha: f64) -> String {

    get_hsl_from_vec([obj[1], obj[2]], alpha)

}

pub fn get_object_color_from_speed(obj: &Vec<f64>, alpha: f64) -> String {

    const MAX_SPEED: f64 = 1.5;
    let u = obj[3];
    let v = obj[4];
    let speed = (u.powf(2.) + v.powf(2.)).sqrt();
    let foo = f64::min(1., speed / MAX_SPEED) * 255.;

    // TODO generalize gradients
    let r = foo;
    let g = 255. - (255. * (foo-127.).abs()/128.);  
    let b = 255. - foo;
    format!("rgba({}, {}, {}, {})", r, g, b, alpha)

}

pub fn get_object_color_from_mass(obj: &Vec<f64>, alpha: f64) -> String {

    const MAX_MASS: f64 = 1.5;
    let m = obj[0];
    let foo = f64::min(1., m / MAX_MASS) * 255.;

    // TODO generalize gradients
    let r = foo;
    let g = 255. - (255. * (foo-127.).abs()/128.);  
    let b = 255. - foo;
    format!("rgba({}, {}, {}, {})", r, g, b, alpha)
    // String::from("rgba(255, 255, 255, 1)")

}

pub fn get_object_color_from_charge(obj: &Vec<f64>, alpha: f64) -> String {

    const Q_MAX: f64 = 1.; // TODO
    let mut charge = 0f64;
    if obj.len() > 5 {
        charge = obj[5]; // TODO max charge 
    }
    let x = (charge / Q_MAX + 1.) * 255. / 2.;
    let r = x; // flip blue & red
    let g = 255. - (255. * (r-127.).abs()/128.);
    let b = 255. - r;
    format!("rgba({}, {}, {}, {})", r, g, b, alpha)

}

pub fn get_object_color_from_distance(obj: &Vec<f64>, alpha: f64) -> String {

    const MAX_DIST: f64 = 1.;
    let x = obj[1];
    let y = obj[2];
    let dist = (x.powf(2.) + y.powf(2.)).sqrt();
    let foo = f64::min(1., dist / MAX_DIST) * 255.;

    let r = foo;
    let r = 255. - r; // flip blue & red
    let g = 255. - (255. * (r-127.).abs()/128.);
    let b = 255. - r;
    format!("rgba({}, {}, {}, {})", r, g, b, alpha)

}

pub fn get_object_color_default(_obj: &Vec<f64>, alpha: f64) -> String {
    format!("rgba(255, 255, 255, {})", alpha)
}

