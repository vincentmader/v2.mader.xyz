use rand::{Rng};

use crate::utils;
use crate::utils::physics::v_kepler;


pub struct Engine {
    pub state: Vec<f64>,
    page_id: String,
    // TODO: define for smaller set of bodies?
    interaction_variant: Interaction,
    integrator_variant: Integrator, 
}
impl Engine {
    pub fn new(
        page_id: &str
    ) -> Self {
        // choose relevant interaction
        let interaction_variant = match page_id {
            "charge-interaction" => Interaction::Coulomb,
            _ => Interaction::NewtonianGravity
        };
        // choose relevant integrator
        let integrator_variant = match page_id {
            _ => Integrator::EulerExplicit
        };
        // 
        let state: Vec<f64> = Vec::new();
        Engine {
            state,
            page_id: String::from(page_id), 
            interaction_variant,
            integrator_variant,
        }
    }
    pub fn init(&mut self) {
        // initialize state vector
        self.state = match self.page_id.as_str() {
            "charge-interaction" 
                => get_initial_state_charge_interaction(&self.page_id), 
            "ising" 
                => get_initial_state_cellauto(&self.page_id), 
            _ => get_initial_state_nbody(&self.page_id),
        }
    }
    pub fn step(&mut self) {
        self.update_bodies();
        // self.state = match self.page_id.as_str() {
        //     "ising" 
        //         => step_cellauto(&self.page_id, &self.state),
        //     _ => self.update_bodies()
        //     // "3body-fig8" | "3body-moon" | "nbody-flowers" |
        //     // "nbody-asteroids"
        //         // => step_nbody(&self.page_id, &self.state),
        //         // => self.update_bodies(),
        //     // "charge-interaction" 
        //         // => step_charge_interaction(&self.page_id, &self.state),
        //         // => self.update_bodies(),
        // };
    }
    pub fn update_bodies(&mut self) -> Vec<f64> {

        // setup numerical parameters
        let (mut dt, mut eps) = (0., 0.);
        match self.page_id.as_str() {
            "charge-interaction" => {
                dt = 0.0005;
                eps = 0.1;
            }, "nbody-flowers" => {
                dt = 0.01;
                eps = 0.1;
            }, "nbody-asteroids" => {
                dt = 0.002;
                eps = 0.;
            }, "3body-fig8" => {
                dt = 0.01;
                eps = 0.;
            }, "3body-moon" => {
                dt = 0.002;
                eps = 0.001;
            }, _ => {
                dt = 0.01;
                eps = 0.;
            }
        };
        // setup boundary variant
        let boundary_variant = match self.page_id.as_str() {
            "charge-interaction" => Boundary::Elastic,
            _ => Boundary::Null
        };

        // setup interaction-force variant
        let interaction_force = match self.interaction_variant {
            Interaction::Coulomb => utils::physics::force_coulomb,
            Interaction::NewtonianGravity => utils::physics::force_newton,
            _ => utils::physics::force_coulomb // TODO
        };
        // setup integrator variant
        let integrator = match self.integrator_variant {
            // Integrator::EulerExplicit => utils::math::integrators::euler_exp,
            // Integrator::EulerImplicit => utils::math::integrators::euler_imp,
            // Integrator::LeapFrog => utils::math::integrators::leap_frog,
            // Integrator::Verlet => utils::math::integrators::verlet,
            // Integrator::RungeKutta2 => utils::math::integrators::runge_kutta_2,
            // Integrator::RungeKutta4 => utils::math::integrators::runge_kutta_4,
            _ => utils::math::integrators::euler_exp,
        };

        // setup length of individual body &Vec slice
        let n = match self.interaction_variant {
            Interaction::Coulomb => 6,
            _ => 5, 
        };

        // setup quad-tree (or skip)
        let using_quad_tree = false;
        let quad_tree = match using_quad_tree {
            false => QuadTree::new(&Vec::new()),
            true => QuadTree::new(&self.state)  // TODO: populate quad-tree
        };

        // loop over all bodies & append updated to new_state
        let mut new_state: Vec<f64> = Vec::new();
        let nr_of_bodies = self.state.len() / n;
        for body_idx in 0..nr_of_bodies {

            // setup "list" of relevant neighbors/attractors (from qt, or all)
            let other_indices = match using_quad_tree {
                true => quad_tree.walk(body_idx),
                false => (0..nr_of_bodies).collect()
            };

            // loop over all attractors & apply interaction
            let mut body = Vec::from(self.state.get(n*body_idx..n*(body_idx+1)).unwrap());
            for other_idx in other_indices.iter() {
                // no self-interaction
                if body_idx == *other_idx { continue }
                let other = Vec::from(self.state.get(n*other_idx..n*(other_idx+1)).unwrap());
                // apply integrator
                integrator(&mut body, &other, &interaction_force, dt, eps);
            }

            // update body position
            body[1] += body[3] * dt;
            body[2] += body[4] * dt;
            apply_bounds(&mut body, &boundary_variant);

            // append to new state
            new_state.extend_from_slice(&body);
                // utils::dom::console_log(&format!("{}", other_idx));
        }
        new_state
    }
}

fn apply_bounds(body: &mut Vec<f64>, boundary_variant: &Boundary) {
    let mut x = body[1];
    let mut y = body[2];
    let mut u = body[3];
    let mut v = body[4];
    match boundary_variant {
        Boundary::Null => {},
        Boundary::Periodic => {
           if x < 0. { x+=1. }
           else if x > 1. { x-=1. }
           if y < 0. { y+=1. }
           else if y > 1. { y-=1. }
        },
        Boundary::Elastic => {
           if x < 0. { x=0.; u*=-1. }
           else if x > 1. { x=1.; u*=-1. }
           if y < 0. { y=0.; v*=-1. }
           else if y > 1. { y=1.; v*=-1. }
        }
        Boundary::Stop => {
           if x < 0. { x=0.; u=0.; v=0. }
           else if x > 1. { x=1.; u=0.; v=0. }
           if y < 0. { y=0.; u=0.; v=0. }
           else if y > 1. { y=1.; u=0.; v=0. }
        }
    }
    body[1] = x;
    body[2] = y;
    body[3] = u;
    body[4] = v;
}

struct QuadTree {

}
impl QuadTree {
    fn new(state: &Vec<f64>) -> Self {
        if state.len() > 0 {
            // TODO: populate tree
        }
        QuadTree {
            // nodes: Vec<>
        }
    }
    fn walk(&self, body_idx: usize) -> Vec<usize> {
        // TODO: walk tree
        Vec::from([0])
    }
}
enum Boundary {
    Periodic,
    Elastic,
    Null,
    Stop,
}
enum Integrator {
    EulerExplicit,
    EulerImplicit,
    LeapFrog,
    Verlet,
    RungeKutta2,
    RungeKutta4,
}
enum Interaction {
    Null,
    // ----------------- body interactions
    Coulomb,
    NewtonianGravity,
    // Boid,
    // Ant,
    // Collision,
    // ----------------- cellular automata
    // ForceField, // (?)
    Fluid,
    Ising,
    GameOfLife,
}
fn update_fields(
    state: &Vec<f64>,
    interaction_variant: Interaction,
) {
    // TODO: move the following into function parameters
    let grid_size = 10; 
    let nr_of_bodies = 1;

    for row_idx in 0..grid_size {
        let y = row_idx as f64 / grid_size as f64;
        for cell_idx in 0..grid_size {
            let x = cell_idx as f64 / grid_size as f64;
            match interaction_variant {
                // Interaction::ForceField => {
                //     // let (m, )
                //     for body_idx in 0..nr_of_bodies {
                //         // interaction(&Vec::from([0.]))
                //     }
                // }
                _ => {} // TODO: raise
            }
        }
    }
}


























pub fn step_charge_interaction(page_id: &str, state: &Vec<f64>) -> Vec<f64> {
    let mut rng = rand::thread_rng();

    let mut new_state: Vec<f64> = Vec::new();

    let nr_of_bodies = 7; // TODO
    let n = 6; // TODO: think about relevant nbody parameters
    let dt = 0.0001;
    let using_periodic_bounds = false;
    let K = 1.;
    let eps: f64 = 0.05;
    let k = 0.01;

    // update charged particles
    for id in 0..nr_of_bodies {

        let m = state[n*id];
        let mut x = state[n*id+1];
        let mut y = state[n*id+2];
        let mut u = state[n*id+3];
        let mut v = state[n*id+4];
        let q = state[n*id+5];

        for jd in 0..nr_of_bodies {
            if id == jd { continue }

            let X = state[n*jd+1];
            let Y = state[n*jd+2];
            // let U = state[n*jd+3];
            // let V = state[n*jd+4];
            let Q = state[n*jd+5];

            let dist = ((X-x).powf(2.) + (Y-y).powf(2.)).sqrt();
            let force = -K * q * Q / (dist.powf(2.) + eps.powf(2.));
            
            u += k * force * (X-x)/dist / m;
            v += k * force * (Y-y)/dist / m;
        }
        x += u*dt;
        y += v*dt;

        // periodic boundaries
        if using_periodic_bounds {
            if x < 0. { x+=1. }
            else if x > 1. { x-=1. }
            if y < 0. { y+=1. }
            else if y > 1. { y-=1. }
        } else {
            if x < 0. { x=0.; u*=-1. }
            else if x > 1. { x=1.; u*=-1. }
            if y < 0. { y=0.; v*=-1. }
            else if y > 1. { y=1.; v*=-1. }
        }
        new_state.extend_from_slice(&[m, x, y, u, v, q]);
    }
    // update field(s?) TODO: multiple fields?
    let grid_size = 35;  // nr. of cells per row
    for y in 0..grid_size {
        for x in 0..grid_size {
            let mut Fx = 0.;
            let mut Fy = 0.;

            let grid_size = grid_size as f64;
            let x = x as f64 / grid_size;
            let y = y as f64 / grid_size;

            let body = [0., 1., x, y, 0., 0.];

            for id in 0..nr_of_bodies {

                let X = new_state[n*id+1];
                let Y = new_state[n*id+2];
                let Q = new_state[n*id+5];

                // let other = new_state.get(n*id..n*(id+1)).unwrap();
                // let other = [
                //     new_state[n*id],
                //     new_state[n*id+1],
                //     new_state[n*id+2],
                //     new_state[n*id+3],
                //     new_state[n*id+4],
                //     new_state[n*id+5],
                // ];

                let dist = ((X-x).powf(2.) + (Y-y).powf(2.)).sqrt();
                let force = -K * Q / (dist.powf(2.) + eps.powf(2.));

                let force_x = force * (X-x)/dist;
                let force_y = force * (Y-y)/dist;

                // let force = -K * Q / dist.powf(2.);
                // let force = utils::physics::force_coulomb(
                //     &body, 
                //     &other
                // );
                // Fx += force.0;
                // Fy += force.1;
                Fx += force_x;
                Fy += force_y;
            }
            new_state.extend_from_slice(&[Fx, Fy]);
        }
    }
    new_state
}

// pub fn step_nbody(page_id: &str, state: &Vec<f64>) -> Vec<f64> {

//     let n = 5;
//     let N = state.len() / n;

//     let G: f64 = 0.01;
//     let eps: f64 = 0.00;
//     let dt: f64 = 0.05;

//     let mut state_clone = state.clone();

//     for body_id in 0..N {
//         let x1 = state[n*body_id+1];
//         let y1 = state[n*body_id+2];
//             // let m1 = state[n*body_id];
//             // let u1 = state[n*body_id+3];
//             // let v1 = state[n*body_id+4];

//         let mut acc_x = 0.;
//         let mut acc_y = 0.;
//         for body_jd in 0..N {
//             // no self-interaction
//             if body_id == body_jd {
//                 continue
//             }
//             // read from state vector
//             let x2 = state[n*body_jd+1];
//             let y2 = state[n*body_jd+2];
//             let m2 = state[n*body_jd];
//                 // let u2 = state[n*body_jd+3];
//                 // let v2 = state[n*body_jd+4];
//             // get relative placement
//             let delta_x = x2 - x1;
//             let delta_y = y2 - y1;
//             let dist = (delta_x.powf(2.)+delta_y.powf(2.)).sqrt();
//             let unit_x = delta_x / dist;
//             let unit_y = delta_y / dist;
//             // euler
//             acc_x += G*m2 / (dist.powf(2.) + eps.powf(2.)) * unit_x;
//             acc_y += G*m2 / (dist.powf(2.) + eps.powf(2.)) * unit_y;
//         }
//         state_clone[n*body_id + 3] += acc_x * dt;
//         state_clone[n*body_id + 4] += acc_y * dt;    
//     }

//     // let mut state_clonee = state_clone.clone();
//     for body_id in 0..N {
//         let u = state_clone[n*body_id + 3];
//         let v = state_clone[n*body_id + 4];
//         state_clone[n*body_id + 1] += u * dt;
//         state_clone[n*body_id + 2] += v * dt;
//         // if body_id == 0 {
//         // console_log(&format!("{}", u));
//         // }
//     }

//     state_clone
// }

pub fn step_cellauto(page_id: &str, state: &Vec<f64>) -> Vec<f64>{
    const M: usize = 1000;  // TODO: make changeable

    let mut rng = rand::thread_rng();

    let mut state_clone = state.clone();
    let N = (state.len() as f64).sqrt() as usize; // TODO: only works for square

    let mut cell_ids: Vec<usize> = vec![];
    for _ in 0..M {
        cell_ids.push(rng.gen_range(0..N.pow(2)));
    }

    let k = 1.;
    let mu = 1.;
    let J = 1.;
    let B = 0.;  // TODO: make changeable
    let T = 1.;  // TODO: make changeable
    let beta = k * T;

    for cell_id in cell_ids.iter() {

        let spin = state[*cell_id];
        let y = cell_id / N;
        let x = cell_id % N;

        let mut dE = 0.;
        let arr: [i32; 3] = [-1, 0, 1];
        for dy in arr.iter() {
            for dx in arr.iter() {
                // prevent self-self interaction
                if (*dx == 0) && (*dy == 0) {
                   continue
                }
                // apply periodic boundaries
                let mut a = x as i32 + dx;  // new coord
                let mut b = y as i32 + dy;
                let n = N as i32;
                if a < 0 {a += n} else if a >= n {a -= n}
                if b < 0 {b += n} else if b >= n {b -= n}
                let a = a as usize;  // convert back
                let b = b as usize;
                // get neighboring spin value
                let other = &state[b*N+a];
                // apply energy change
                dE -= -J * spin * other;
                dE += -J * -spin * other;
           }
        }
        // change in energy through flip
        dE -= B*mu*spin;
        dE += B*mu*-spin;
        // probability for flip
        let p = (-dE * beta).exp();
        let rand: f64 = rng.gen();
        if rand < p {
            state_clone[y*N+x] *= -1.;
        }
    }
    state_clone
}































pub fn get_initial_state_nbody(page_id: &str) -> Vec<f64> {
    const TAU: f64 = 2.*3.14159;
    let mut rng = rand::thread_rng();

    let mut state: Vec<f64> = Vec::new();

    // let k = 0.1; // TODO: why?
    let k = 10.; // TODO: why?

    match page_id {
        "2body-kepler" => {

        }, "3body-moon" => {
            let (r, dr) = (0.8, 0.2);
            // add Sun
            let (M, x, y, u, v) = (1.0, 0., 0., 0., 0.);
            state.extend_from_slice(&[M, x, y, u, v]);
            // add Earth
            let v = k*v_kepler(M, r);  // TODO: add G*M vars?
            let (m, x, y, u, v) = (1.0e-2, r, 0., 0., v);
            state.extend_from_slice(&[m, x, y, u, v]);
            // add Moon
            let v = k*v_kepler(M, r+dr) + k*v_kepler(m, dr);
            let (m, x, y, u, v) = (1.0e-4, r+dr, 0., 0., v);
            state.extend_from_slice(&[m, x, y, u, v]);
            // TODO: fix Moon orbit
            //  - what happened?
            //  - move to mass-less particles for moons/rings?

        }, "3body-fig8" => {
            let m = 1.;
            // body 1 & 3
            let (x, y, u, v) = (
               0.7775727187509279270, 0.6287930240184685937, 
               -0.06507160612095737318, 0.6324957346748190101
            );
            state.extend_from_slice(&[m, x, y, u, v]);
            state.extend_from_slice(&[m, -x, -y, u, v]);
            // body 2 (center)
            let (x, y, u, v) = (
                0., 0., 
                0.1301432122419148851, -1.264991469349638020
            );
            state.extend_from_slice(&[m, x, y, u, v]);

        }, "nbody-flowers" => {
            // add Sun
            let (m, x, y, u, v) = (1., 0., 0., 0., 0.);
            state.extend_from_slice(&[m, x, y, u, v]);
            // // add satellites
            let m = 0.;  // TODO: unphysical of course, but stable!
            let r = 0.9;
            let v0 = 0.45;
            let N = 20; // TODO: make changeable
            for id in 0..N {
                let phi = id as f64 / N as f64 * TAU;
                let x = r * phi.cos();
                let y = r * phi.sin();
                let u = -v0 * phi.sin();
                let v = v0 * phi.cos();
                state.extend_from_slice(&[m,x,y,u,v]);
            }

        }, "nbody-asteroids" => {
            // add binary stars
            let (M, r, v0) = (1., 0.1, 1.575);
            let v0 = v_kepler(0.1*M, r);
            let (m, x, y, u, v) = (M, r, 0., 0., v0);
            state.extend_from_slice(&[m, x, y, u, v0]);
            state.extend_from_slice(&[m, -M/m*x, y, u, -v0]);

            // add satellites
            let N = 100; // TODO: make changeable
            for id in 0..N {
                // let mut phi: f64 = rng.gen();
                // phi *= TAU;
                // let mut r: f64 = rng.gen();
                // r *= 0.8;
                // r += 0.5;
                let r = 0.8;
                let phi = id as f64 / N as f64 * TAU;
                // let r = 0.42 + 0.44 * id as f64 / N as f64;;
                let v0 = v_kepler(2., r);
                let m = 0.;  // TODO: unphysical of course, but stable!
                let x = r * phi.cos();
                let y = r * phi.sin();
                let u = -v0 * phi.sin();
                let v = v0 * phi.cos();
                state.extend_from_slice(&[m, x, y, u, v]);
            }
            // let r = 0.6;
            // let m = 1e-3;
            // let (m, x, y, u, v) = (m, r, 0., 0., -v_kepler(2., r));
            // state.extend_from_slice(&[m, x, y, u, v]);
            // let dr = 0.02;
            // let (m, x, y, u, v) = (0., r+dr, 0., 0., -(v_kepler(2., r+dr)+v_kepler(m, dr)));
            // state.extend_from_slice(&[m, x, y, u, v]);

        }, _ => {} // TODO: raise exc.?
    }
    state
}

pub fn get_initial_state_cellauto(page_id: &str) -> Vec<f64> {
    let mut rng = rand::thread_rng();

    // TODO: append
    let foo: u32 = match page_id {
        "ising" => 150,
        // "charge-interaction" => 150,
        _ => 16,  // TODO: ?
    };
    let nr_of_cells = foo.pow(2);

    let mut state: Vec<f64> = Vec::new();
    for _ in 0..nr_of_cells {
        let spin_up: bool = rng.gen();
        if spin_up {
            state.push(1.);
        } else {
            state.push(-1.);
        }
    }
    state
}


pub fn get_initial_state_charge_interaction(page_id: &str) -> Vec<f64> {
    let mut rng = rand::thread_rng();

    let mut state: Vec<f64> = Vec::new();

    // initialize bodies
    let nr_of_bodies = 10;

    for id in 0..nr_of_bodies {
        let positive_charge: bool = rng.gen();
        let q = match positive_charge {
            true => 1.,
            false => -1.,
        };
        let m = match positive_charge {
            true => 1.,
            false => 0.1,
        };
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        let (m, x, y, u, v, q) = (m, x, y, 0., 0., q);
        state.extend_from_slice(&[m, x, y, u, v, q]);
    }

    // let a = f64::from(nr_of_bodies).sqrt();
    // let a = (a - a % 1.) as u32;
    // for foo in 0..a {
    //     for bar in 0..a {
    //         let (q, x, y, u, v) = (
    //             1., 
    //             f64::from(foo) / f64::from(a) + 0.5/f64::from(a), 
    //             f64::from(bar) / f64::from(a) + 0.5/f64::from(a), 
    //             0., 
    //             0.
    //         );
    //         state.extend_from_slice(&[q, x, y, u, v]);
    //     }
    // }

    // initialize grid
    // let grid_size = 35;
    // for _ in 0..grid_size {
    //     let Fx = 0.;
    //     let Fy = 0.; // TODO:
    //     state.extend_from_slice(&[Fx, Fy]);
    // }

    state
}
