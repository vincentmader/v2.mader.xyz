use rand::Rng;

pub mod body;
use body::Body;
use crate::utils::Vector2D;



pub fn euler_solve(state: &Vec<Body>) -> Vec<Body> {

    let mut state_clone = state.clone();
    let N = state.len();

    let G = 1.;
    let dt = 1.;

    for id in 0..N {
        for jd in 0..N {
            if id == jd { continue }
            
            let eps: f64 = 0.1;

            // let m1 = state[id].mass;
            let m2 = state[jd].mass;
            let pos1 = state[id].position;
            let pos2 = state[jd].position;
            let vel1 = state[id].velocity;
            // let vel2 = state[jd].velocity;

            let conn = pos2 - pos1;
            let dist = conn.norm_l2();
            let unit = conn / dist;
            let r2 = dist.powf(2.) + eps.powf(2.);

            let acc = G * m2 / r2 * unit;

            state_clone[id].velocity = vel1 + acc * dt;
        }
    }

    state_clone
}



pub struct Engine {
    pub state: Vec<Body>,
    page_id: String,
}
impl Engine {
    pub fn new(page_id: String) -> Self {
        let state: Vec<Body> = vec![];
        Engine {
            state, page_id,
        }
    }
    pub fn init(&mut self) {
        let mut rng = rand::thread_rng();

        let mass = 1.;

        let N: usize = 300;
        for id in 0..N {

            let mut phi: f64 = rng.gen();
            phi *= 2. * 3.14159;
            let mut phi2: f64 = rng.gen();
            phi2 *= 2. * 3.14159;
            let mut r0: f64 = rng.gen();
            r0 *= 0.5;
            
            let position = Vector2D::new(
                r0 * phi2.cos(), r0 * phi2.sin()
            );
            let velocity = Vector2D::new(
                // phi.cos(), phi.sin()
                0., 0.
            );

            self.state.push(Body::new(
                id as u32, mass, position, velocity
            ));
        }
    }
    pub fn step(&mut self) {
        let mut rng = rand::thread_rng();

        let state_2 = &self.state.clone();

        let dt: f64 = 0.001;

        // self.state = euler_solve(&self.state);


        let N: usize = 300;
        for id in 0..N {

            // for jd in 0..N {
            //     if id == jd { continue }
                // self.update_velocity()
            // }

            self.state[id].update_velocity(&state_2, dt);
            self.state[id].update_position(dt);
            
            // let position = Vector2D::new(0., 0.);
            // let velocity = Vector2D::new(1., 0.);

            // self.state.push(Body::new(
            //     id as u32, mass, position, velocity
            // ));
        }
    }
}


// // use crate::universe::Universe;
// use crate::simulation::universe::Universe;
// // use super::universe::Universe;
// use crate::utils::Vector2D;

// // pub struct EulerExplicitSolver {

// // }
// // impl EulerExplicitSolver {
// //     pub fn new(body: Body, bodies: Vec<Body>) -> Self {
// //         EulerExplicitSolver {

// //         }
// //     }
// //     pub fn init(&self) {

// //     }
// // }

// // pub enum EngineType {
// //     NBodySolver,
// // }

// // pub enum NBodySolverType {
// //     EulerExplicit(EulerExplicitSolver),
// //     // EulerImplicit,
// //     // RungeKutta2,
// //     // RungeKutta4,
// //     // LeapFrog,
// //     // Verlet,
// // }


// pub struct Engine {
//     // engine_type: EngineType,
//     // solver_type: NBodySolverType,
//     pub universe: Universe,
//     dt: f64,
//     step_idx: u32,
// }
// impl Engine {
//     pub fn new(nr_of_bodies: u32, dt: f64) -> Self {

//         // let engine_type = EngineType::NBodySolver;
//         // let solver_type = NBodySolverType::EulerExplicit;
//         let universe = Universe::new(nr_of_bodies, dt); // TODO -dt

//         Engine {
//             // solver_type, engine_type, 
//             universe, dt,
//             step_idx: 0, // TODO: different to that in Simulation
//         }
//     }
//     pub fn init(&mut self) {
//         self.universe.init();
//     }
//     pub fn step(&mut self) {

//         // for &mut entity in self.universe.entities {
//         //     entity.step();
//         // }

//         let solver = "EulerExplicit";

//         let bodies2 = self.universe.bodies.clone();

//         if solver == "EulerExplicit" {
//             for body in &mut self.universe.bodies {
//                 let mut a_i = Vector2D::new(0., 0.);
//                 for other in &bodies2 {
//                     a_i = a_i + body.get_acceleration(other);
//                 }
//                 // EulerExplicit
//                 body.velocity = body.velocity + a_i * self.dt; 
//                 body.position = body.position + body.velocity * self.dt;
//             }
//         } else if solver == "LeapFrog" {
//             // LeapFrog   (TODO: update all x_ip1 in loop, then loop again)
//             for body in &mut self.universe.bodies {
//                 let mut a_i = Vector2D::new(0., 0.);
//                 for other in &bodies2 {
//                     a_i = a_i + body.get_acceleration(other);
//                 }
//                 let x_i = body.position;
//                 let mut v_imh = Vector2D::new( 0., 0. ); // TODO
//                 if self.step_idx == 0 {
//                     v_imh = body.velocity + a_i / 2.;
//                 } else {
//                     v_imh = body.velocity;
//                 }
//                 let x_ip1 = x_i + v_imh * self.dt;
//                 body.position = x_ip1;
//             } // TODO: initial conditions?  -> Euler, Mid-point or Rk4
//             for body in &mut self.universe.bodies {
//                 let mut a_i = Vector2D::new(0., 0.);
//                 for other in &bodies2 {
//                     a_i = a_i + body.get_acceleration(other);
//                 }
//                 let v_imh = body.velocity;
//                 let v_iph = v_imh + a_i * self.dt;
//                 body.velocity = v_iph;

//                 // let v_imh = body.velocity - a_i * self.dt / 2.;


//                 // let x = body.position.x;
//                 // let array = js_sys::Array::new();
//                 // array.push(&x.into());
//                 // web_sys::console::log(&array);
//             }
//         }
//         self.step_idx += 1;
//     }
// }
