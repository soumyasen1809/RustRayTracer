use super::vector3::Vector3;
use std::ops::{Add, Neg};

/// The Point Class
///

#[derive(Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn get_x(self) -> f64 {
        self.x
    }
    pub fn get_y(self) -> f64 {
        self.y
    }
    pub fn get_z(self) -> f64 {
        self.z
    }
}

impl Default for Point {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add<Vector3> for Point {
    type Output = Self;
    fn add(self, rhs: Vector3) -> Self::Output {
        Self {
            x: self.x * rhs.get_x(),
            y: self.y * rhs.get_y(),
            z: self.z * rhs.get_z(),
        }
    }
}
