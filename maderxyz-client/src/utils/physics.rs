
// pub fn f_euler(id: u32, state: &Vec<f64>) {
//     let N = state.len();
//     for other_id in 0..N {
        
//     }
// }

// pub enum IntegratorType {
//     Euler,
//     Verlet,
//     RungeKutta2,
//     RungeKutta4,
//     LeapFrog,
// }
// pub struct Integrator {
//     integrator_type: IntegratorType,
// }
// impl Integrator {
//     fn new(page_id: &str) -> Self {

//         let integrator_type = match page_id {
//             _ => IntegratorType::Euler
//         };

//         Integrator {
//             integrator_type
//         }
//     }
//     fn init(&mut self) {}
//     fn step(&mut self, state: &Vec<f64>) -> Vec<f64> {
//         // let f = match self.integrator_type {
//         //     IntegratorType::Euler => f_euler,
//         //     _ => f_euler,
//         // }

//         let new_state = Vec::new();

//         let N = state.len();
//         let n = 5;

//         for id in 0..N {

//             let m = state[n*id];
//             let x = state[n*id+1];
//             let y = state[n*id+2];
//             let u = state[n*id+3];
//             let v = state[n*id+4];

//             for jd in 0..N {
                
//             }

//         }
//         new_state
//     }
// }

// TODO: actually: fields! not forces
pub fn force_coulomb(body: &Vec<f64>, other: &Vec<f64>, eps: f64) -> (f64, f64) {
    // operates on bodies of type [f64; 6] = [m,x,y,u,v,q]
    let (q, x, y) = (body[5], body[1], body[2]);
    let (Q, X, Y) = (other[5], other[1], other[2]);
    let dist = ((X-x).powf(2.) + (Y-y).powf(2.)).sqrt();
    // 
    // const k: f64 = 1.;
    let force = -Q / (dist.powf(2.) + eps.powf(2.)); // k:=1.
    let force_x = force * (X-x)/dist;
    let force_y = force * (Y-y)/dist;
    (force_x, force_y)
}

pub fn force_newton(body: &Vec<f64>, other: &Vec<f64>, eps: f64) -> (f64, f64) {
    // operates on bodies of type [f64; 5] = [m,x,y,u,v]
    let (m, x, y) = (body[0], body[1], body[2]);
    let (M, X, Y) = (other[0], other[1], other[2]);
    let dist = ((X-x).powf(2.) + (Y-y).powf(2.)).sqrt();
    // 
    // const G: f64 = 1.;
    let force = M / (dist.powf(2.) + eps.powf(2.)); // G:=1.
    let force_x = force * (X-x)/dist;
    let force_y = force * (Y-y)/dist;
    (force_x, force_y)
}

pub fn v_kepler(M: f64, r: f64) -> f64 {
    (M/r).sqrt() // G:=1.
}
