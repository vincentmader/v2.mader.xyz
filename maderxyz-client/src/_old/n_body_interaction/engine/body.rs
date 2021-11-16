use crate::utils::Vector2D;


#[derive(Clone)]
pub struct Body {
    pub id: u32,
    pub mass: f64,
    pub position: Vector2D,
    pub velocity: Vector2D,
}
impl Body {
    pub fn new(
        id: u32,
        mass: f64, 
        position: Vector2D, 
        velocity: Vector2D
    ) -> Self {
        Body { id, mass, position, velocity }
    }
    pub fn update_position(&mut self, dt: f64) {
        self.position = self.position + self.velocity * dt;
    }
    pub fn update_velocity(&mut self, state: &Vec<Body>, dt: f64) {
        for id in 0..state.len() {
            if id as u32 == self.id { continue }

            let G = 1.;
            let eps: f64 = 1.;
            // let m1 = self.mass;
            let m2 = state[id].mass;

            let connection = state[id].position - self.position;
            let distance = connection.norm_l2();
            let unit = connection / distance;
            let r2 = distance.powf(2.)+eps.powf(2.);
            let acc = G * m2 / r2 * unit;

            self.velocity = self.velocity + acc * dt;
        }
        // self.velocity = self.velocity + acceleration * dt;
    }
    // TODO: remove the following
    pub fn get_acceleration(&self, other: &Body) -> Vector2D {
        if self.id == other.id {
            return Vector2D { x: 0., y: 0. };
        }
        const G: f64 = 1.; // TODO
        let epsilon: f64 = 0.0; // = 0.0;
        // let epsilon: f64 = 0.1; // = 0.0;

        let unit = other.position - self.position;
        let distance = unit.norm_l2();
        let acc = G * other.mass * unit / distance / (distance+epsilon).powf(3.);
        acc
    }
}

