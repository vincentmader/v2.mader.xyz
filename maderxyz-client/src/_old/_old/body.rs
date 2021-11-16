
use crate::utils::math::Vector2D;


#[derive(Clone)]
pub struct Body {
    pub id: u32,
    pub mass: f64,
    pub position: Vector2D,
    pub velocity: Vector2D,
}
impl Body {
    pub fn new(
        id: u32, // TODO: atm, can be used twice
        mass: f64, 
        position: Vector2D, 
        velocity: Vector2D
    ) -> Body {
        Body { 
            id, mass, position, velocity
        }
    }
    pub fn init(&mut self) {

    }
    pub fn get_acceleration(&self, other: &Body) -> Vector2D {
        if self.id == other.id {
            return Vector2D { x: 0., y: 0. };
        }
        const G: f64 = 1.; // TODO
        let epsilon: f64 = 0.001; // = 0.0;
        // let epsilon: f64 = 0.1; // = 0.0;

        let unit = other.position - self.position;
        let distance = unit.norm_l2();
        let acc = G * other.mass * unit / (distance+epsilon).powf(3.);
        acc
    }
}
