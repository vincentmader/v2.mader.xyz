// use crate::universe::Universe;
use crate::simulation::universe::Universe;
// use super::universe::Universe;
use crate::utils::Vector2D;

// pub struct EulerExplicitSolver {

// }
// impl EulerExplicitSolver {
//     pub fn new(body: Body, bodies: Vec<Body>) -> Self {
//         EulerExplicitSolver {

//         }
//     }
//     pub fn init(&self) {

//     }
// }

// pub enum EngineType {
//     NBodySolver,
// }

// pub enum NBodySolverType {
//     EulerExplicit(EulerExplicitSolver),
//     // EulerImplicit,
//     // RungeKutta2,
//     // RungeKutta4,
//     // LeapFrog,
//     // Verlet,
// }


pub struct Engine {
    // engine_type: EngineType,
    // solver_type: NBodySolverType,
    pub universe: Universe,
    dt: f64,
    step_idx: u32,
}
impl Engine {
    pub fn new(nr_of_bodies: u32, dt: f64) -> Self {

        // let engine_type = EngineType::NBodySolver;
        // let solver_type = NBodySolverType::EulerExplicit;
        let universe = Universe::new(nr_of_bodies, dt); // TODO -dt

        Engine {
            // solver_type, engine_type, 
            universe, dt,
            step_idx: 0, // TODO: different to that in Simulation
        }
    }
    pub fn init(&mut self) {
        self.universe.init();
    }
    pub fn step(&mut self) {

        // for &mut entity in self.universe.entities {
        //     entity.step();
        // }

        let solver = "EulerExplicit";

        let bodies2 = self.universe.bodies.clone();

        if solver == "EulerExplicit" {
            for body in &mut self.universe.bodies {
                let mut a_i = Vector2D::new(0., 0.);
                for other in &bodies2 {
                    a_i = a_i + body.get_acceleration(other);
                }
                // EulerExplicit
                body.velocity = body.velocity + a_i * self.dt; 
                body.position = body.position + body.velocity * self.dt;
            }
        } else if solver == "LeapFrog" {
            // LeapFrog   (TODO: update all x_ip1 in loop, then loop again)
            for body in &mut self.universe.bodies {
                let mut a_i = Vector2D::new(0., 0.);
                for other in &bodies2 {
                    a_i = a_i + body.get_acceleration(other);
                }
                let x_i = body.position;
                let mut v_imh = Vector2D::new( 0., 0. ); // TODO
                if self.step_idx == 0 {
                    v_imh = body.velocity + a_i / 2.;
                } else {
                    v_imh = body.velocity;
                }
                let x_ip1 = x_i + v_imh * self.dt;
                body.position = x_ip1;
            } // TODO: initial conditions?  -> Euler, Mid-point or Rk4
            for body in &mut self.universe.bodies {
                let mut a_i = Vector2D::new(0., 0.);
                for other in &bodies2 {
                    a_i = a_i + body.get_acceleration(other);
                }
                let v_imh = body.velocity;
                let v_iph = v_imh + a_i * self.dt;
                body.velocity = v_iph;

                // let v_imh = body.velocity - a_i * self.dt / 2.;


                // let x = body.position.x;
                // let array = js_sys::Array::new();
                // array.push(&x.into());
                // web_sys::console::log(&array);
            }
        }
        self.step_idx += 1;
    }
}
