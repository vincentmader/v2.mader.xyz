use std::ops; // for operator overload

use crate::utils;
use crate::utils::dom::console_log;


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

pub mod integrators {

    pub fn euler_exp(
        body: &mut Vec<f64>, 
        other: &Vec<f64>, 
        force: &fn(&Vec<f64>, &Vec<f64>, f64) -> (f64, f64),
        dt: f64,
        eps: f64,
    ) {
        let force = force(&body, &other, eps);
        let acc_x = force.0;
        let acc_y = force.0;
        let m = body[0];
        // if m != 0. {
        //     body[3] /= m;
        //     body[4] /= m;
        // }
        body[3] += acc_x * dt;  // u
        body[4] += acc_y * dt;  // v
    }

    pub fn euler_imp(
        body: &mut Vec<f64>, 
        other: &Vec<f64>, 
        force: fn(&Vec<f64>, &Vec<f64>, f64) -> (f64, f64),
        dt: f64,
        eps: f64,
    ) {}

    pub fn verlet(
        body: &mut Vec<f64>, 
        other: &Vec<f64>, 
        force: fn(&Vec<f64>, &Vec<f64>, f64) -> (f64, f64),
        dt: f64,
        eps: f64,
    ) {}

    pub fn leap_frog(
        body: &mut Vec<f64>, 
        other: &Vec<f64>, 
        force: fn(&Vec<f64>, &Vec<f64>, f64) -> (f64, f64),
        dt: f64,
        eps: f64,
    ) {}
    pub fn runge_kutta_2(

        body: &mut Vec<f64>, 
        other: &Vec<f64>, 
        force: fn(&Vec<f64>, &Vec<f64>, f64) -> (f64, f64),
        dt: f64,
        eps: f64,
    ) {}

    pub fn runge_kutta_4(
        body: &mut Vec<f64>, 
        other: &Vec<f64>, 
        force: fn(&Vec<f64>, &Vec<f64>, f64) -> (f64, f64),
        dt: f64,
        eps: f64,
    ) {}

}
