use std::ops; // for operator overload


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2D { pub x: f64, pub y: f64 }
impl Vector2D {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2D { x, y }
    }
    pub fn from_polar(r: f64, phi: f64) -> Self {
        let x = r * phi.cos();
        let y = r * phi.sin();
        Vector2D { x, y }
    }
    pub fn norm_l2(&self) -> f64 {
        (self.x.powf(2.) + self.y.powf(2.)).sqrt()
    }
    pub fn rotate(&self, angle: f64) -> Self {
        let r = (self.x.powf(2.) + self.y.powf(2.)).sqrt();
        let phi = self.y.atan2(self.x) + angle;
        let x = r * phi.cos();
        let y = r * phi.sin();
        Vector2D { x, y }
    }
}
impl ops::Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, _rhs: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}
impl ops::Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, _rhs: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}
impl ops::Mul<f64> for Vector2D {
    type Output = Vector2D;

    fn mul(self, _rhs: f64) -> Vector2D {
        Vector2D {
            x: self.x * _rhs,
            y: self.y * _rhs,
        }
    }
}
impl ops::Mul<Vector2D> for f64 {
    type Output = Vector2D;

    fn mul(self, _rhs: Vector2D) -> Vector2D {
        Vector2D {
            x: self * _rhs.x,
            y: self * _rhs.y,
        }
    }
}
impl ops::Div<f64> for Vector2D {
    type Output = Vector2D;

    fn div(self, _rhs: f64) -> Vector2D {
        Vector2D {
            x: self.x / _rhs,
            y: self.y / _rhs,
        }
    }
}
