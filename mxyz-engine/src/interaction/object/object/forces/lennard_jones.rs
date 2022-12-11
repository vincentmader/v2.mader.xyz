pub fn force(object: &[f64], other: &[f64], epsilon: f64) -> Vec<f64> {
    // atributes of object
    let x_1 = object[1];
    let y_1 = object[2];
    // atributes of other object
    let x_2 = other[1];
    let y_2 = other[2];
    // distance & force
    let dx = x_2 - x_1;
    let dy = y_2 - y_1;
    let r2 = dx.powf(2.) + dy.powf(2.);
    let r = (r2 + epsilon.powf(2.)).sqrt();

    let (sigma, epsilon) = (0.1, 4.); // TODO make changeable, TODO rename epsilon (duplicate name)

    // V = 4 eps ( (s/r)^12 - (s/r)^6 )
    let force = 4. * epsilon * sigma * (6. * (r / sigma).powf(-7.) - 12. * (r / sigma).powf(-13.));
    let force_x = force * dx / r;
    let force_y = force * dy / r;

    Vec::from([force_x, force_y])
}
