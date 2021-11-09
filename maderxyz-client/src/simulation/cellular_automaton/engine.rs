use rand::Rng;


pub struct Engine {
    pub state: Vec<f64>,
    page_id: String,
}
impl Engine {
    pub fn new(page_id: String, N: usize) -> Self {
        let state = vec![0.; N.pow(2)];
        Engine {
            state, page_id,
        }
    }
    pub fn init(&mut self, N: usize) {
        let mut rng = rand::thread_rng();

        for i in 0..N {
            for j in 0..N {

                let spin: f64 = rng.gen();
                if spin > 0.5 {
                    self.state[i*N + j] = 1.;
                } else {
                    self.state[i*N + j] = -1.;
                } // TODO
            }
        }
    }
    // fn get_neighborhood(&self, idx: usize, jdx: usize) {
    //     let mut neighbors: Vec<f64> = vec![];
    // }
    pub fn step(&mut self, N: usize) {
        let mut rng = rand::thread_rng();

        const M: usize = 50;
        let mut ys: Vec<usize> = vec![];
        let mut xs: Vec<usize> = vec![];
        for _ in 0..M {
            xs.push(rng.gen_range(0..N));
            ys.push(rng.gen_range(0..N));
        }

        let k = 1.;
        let mu = 1.;

        let J = 1.;
        let B = 0.;
        let T = 3.;

        let beta = k * T;

        for y in ys.iter() {
            for x in xs.iter() {
                let spin = &self.state[y*N+x];

                let mut dE = 0.;

                let arr: [i32; 3] = [-1, 0, 1];
                for dy in arr.iter() {
                    for dx in arr.iter() {
                        // prevent self-self interaction
                        if (*dx == 0) && (*dy == 0) {
                           continue
                        }
                        // apply periodic boundaries
                        let n = N as i32;
                        let mut a = *x as i32 + dx;  // new coord
                        let mut b = *y as i32 + dy;
                        if a < 0 {a += n} else if a >= n {a -= n}
                        if b < 0 {b += n} else if b >= n {b -= n}
                        let a = a as usize;  // convert back
                        let b = b as usize;
                        // get neighboring spin value
                        let other = &self.state[b*N+a];
                        // apply energy change
                        dE -= -J * spin * other;
                        dE += -J * -spin * other;
                   }
                }
                // change in energy through flip
                dE -= B*mu*spin;
                dE += B*mu*-spin;
                // probability for flip
                let p = (-dE / beta).exp();
                let rand: f64 = rng.gen();
                if rand < p {
                    self.state[y*N+x] *= -1.;
                }
            }
        }
    }
}
