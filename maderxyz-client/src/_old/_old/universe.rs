use rand::Rng;

use super::body::Body;
use crate::utils::math::Vector2D;


pub struct Universe {
    nr_of_bodies: u32,
    pub bodies: Vec<Body>,
    dt: f64,
}
impl Universe {
    pub fn new(
        nr_of_bodies: u32,
        dt: f64
    ) -> Universe {
        let bodies: Vec<Body> = Vec::new();
        Universe {
            nr_of_bodies,
            bodies,
            dt
        }
    }
    pub fn init(&mut self) {
        // self.bodies = bodies_01(self.nr_of_bodies);
        self.bodies = bodies_03();
    }
    // pub fn step(&mut self) {

    // }
}

fn bodies_01(nr_of_bodies: u32) -> Vec<Body> {
    let mut rng = rand::thread_rng();
    let mut bodies: Vec<Body> = Vec::new();

    for body_idx in 0..nr_of_bodies {

        // let mass = 1.;  // TODO: distribution or specified
        let mass = rng.gen_range(0.05..0.5);

        let position = Vector2D { 
            x: rng.gen_range(-1.0..1.), 
            y: rng.gen_range(-1.0..1.) 
        };
        let velocity = Vector2D { 
            x: 0., y: 0.,
            // x: rng.gen_range(-1.0..1.), 
            // y: rng.gen_range(-1.0..1.) 
        };

        let body = Body::new(
            body_idx, 
            mass, 
            position, 
            velocity
        );
        // body.init();

        bodies.push(body);
    }
    bodies
}

fn bodies_02() -> Vec<Body> {
    // let arr: [[f64, Vector2D, Vector2D]; 3] = [
    //     [1.0, Vector2D::new(0.0, 0.0), Vector2D::new(0.0, 0.0)],
    //     [1.0, Vector2D::new(0.0, 0.0), Vector2D::new(0.0, 0.0)],
    //     [1.0, Vector2D::new(0.0, 0.0), Vector2D::new(0.0, 0.0)],
    // ]
    let mut bodies: Vec<Body> = Vec::new();

    const G: f64 = 1.; // TODO fix duplicate definition
    let solar_mass = 1.0;
    let astro_unit = 0.5;
    let vel_Kepler = (G * solar_mass / astro_unit).sqrt();

    let sun = Body::new(
        0, 1.0, Vector2D::new(0.0, 0.0), Vector2D::new(0.0, 0.0)
    );
    let earth = Body::new(
        1, 0.0001, Vector2D::new(astro_unit, 0.0), Vector2D::new(0.0, vel_Kepler)
    );
    let earth2 = Body::new(
        2, 0.0001, Vector2D::new(-astro_unit, 0.0), Vector2D::new(0.0, -vel_Kepler)
    );
    let earth3 = Body::new(
        3, 0.01, Vector2D::new(0.0, astro_unit), Vector2D::new(-vel_Kepler, 0.0)
    );
    let earth4 = Body::new(
        4, 0.0001, Vector2D::new(0.0, -astro_unit), Vector2D::new(vel_Kepler, 0.0)
    );

    // sun.init();
    // earth.init();
    bodies.push(sun);
    bodies.push(earth);
    bodies.push(earth2);
    bodies.push(earth3);
    bodies.push(earth4);

    bodies
}

fn bodies_03() -> Vec<Body> {
    // let arr: [[f64, Vector2D, Vector2D]; 3] = [
    //     [1.0, Vector2D::new(0.0, 0.0), Vector2D::new(0.0, 0.0)],
    //     [1.0, Vector2D::new(0.0, 0.0), Vector2D::new(0.0, 0.0)],
    //     [1.0, Vector2D::new(0.0, 0.0), Vector2D::new(0.0, 0.0)],
    // ]
    let mut bodies: Vec<Body> = Vec::new();
    const PI: f64 = 3.14159; // TODO: move


    // let z: f64 = 0.7; // nice!
    // // let z: f64 = 0.75;
    // let p_1 = Body::new(
    //     0, 1., Vector2D::new(0.0, 0.0), Vector2D::new(2.*z, 0.0).rotate(PI/4.)
    // );
    // let p_2 = Body::new(
    //     1, 1., Vector2D::new(0.0, 1.0), Vector2D::new(-1.*z, 0.0).rotate(PI/4.)
    // );
    // let p_3 = Body::new(
    //     2, 1., Vector2D::new(0.0, -1.0), Vector2D::new(-1.*z, 0.0).rotate(PI/4.)
    // );

    // let p_1 = Body::new(
    //     0, 1., Vector2D::new(0.0, 0.0), Vector2D::new(2.*z, 0.0).rotate(PI/4.)
    // );


    let rotation_angle = -PI/4. - PI/16. + PI/128.;
    // let rotation_angle = -64.35 / 360. * 2. * PI;
    // let rotation_angle = 0.;
    let p_1 = Body::new(0, 1., 
        Vector2D::new(-0.7775727187509279270, -0.6287930240184685937).rotate(rotation_angle), 
        Vector2D::new(-0.06507160612095737318, 0.6324957346748190101).rotate(rotation_angle), 
    );
    let p_2 = Body::new(1, 1., 
        Vector2D::new(0.0, 0.0).rotate(rotation_angle), 
        Vector2D::new(0.1301432122419148851, -1.264991469349638020).rotate(rotation_angle), 
    );
    let p_3 = Body::new(2, 1., 
        Vector2D::new(0.7775727187509279270, 0.6287930240184685937).rotate(rotation_angle),  
        Vector2D::new(-0.06507160612095737318, 0.6324957346748190101).rotate(rotation_angle), 
    );


    // let z = 2.0f64.powf(1./2.)/2.;
    // let z = 0.66;

    // let rotation_angle = PI/4.;
    // let p_1 = Body::new(0, 1., 
    //     Vector2D::new(-1., 0.),  
    //     Vector2D::new(0., -z).rotate(rotation_angle), 
    // );
    // let p_2 = Body::new(1, 1., 
    //     Vector2D::new(0., 0.),  
    //     Vector2D::new(0., 2.*z).rotate(rotation_angle), 
    // );
    // let p_3 = Body::new(2, 1., 
    //     Vector2D::new(1., 0.),  
    //     Vector2D::new(0., -z).rotate(rotation_angle), 
    // );

    // let solar_mass = 1.0;
    // let astro_unit = 0.5;
    // let vel_Kepler = (G * solar_mass / astro_unit).sqrt();
    bodies.push(p_1);
    bodies.push(p_2);
    bodies.push(p_3);

    bodies
}

fn bodies_04() -> Vec<Body> {
    let mut rng = rand::thread_rng();
    let mut bodies: Vec<Body> = Vec::new();


    let M = 1.;

    let sun = Body::new(
        0, 
        M, 
        Vector2D::new(0., 0.), 
        Vector2D::new(0., 0.), 
    );
    bodies.push(sun);

    let nr_of_bodies = 100;
    for body_idx in 0..nr_of_bodies {

        let mass = 0.0001;  // TODO: distribution or specified
        // let mass = rng.gen_range(0.05..0.5);

        let r = 1.;
        let phi = f64::from(body_idx) / f64::from(nr_of_bodies) * 2. * 3.14159;
        let position = Vector2D::from_polar(r, phi);

        let v = kepler_vel(M, r);
        let velocity = Vector2D::from_polar(v, phi + 3.14159 / 2.);

        // let position = Vector2D { 
        //     x: rng.gen_range(-1.0..1.), 
        //     y: rng.gen_range(-1.0..1.) 
        // };
        // let velocity = Vector2D { 
        //     x: 0., y: 0.,
        //     // x: rng.gen_range(-1.0..1.), 
        //     // y: rng.gen_range(-1.0..1.) 
        // };

        let body = Body::new(
            body_idx+1, 
            mass, 
            position, 
            velocity
        );
        // body.init();

        bodies.push(body);
    }
    bodies
}

pub fn kepler_vel(M: f64, r: f64) -> f64 {
    const G: f64 = 1.;
    (G * M / r).sqrt()
}
