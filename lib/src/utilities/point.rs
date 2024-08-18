use std::ops::{Add, Neg, Sub};

use super::vector3::Vector3;

#[derive(Clone, Copy)]
pub struct Point3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3 {
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

    pub fn as_vec(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl Default for Point3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Neg for Point3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Point3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Point3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add<Vector3> for Point3 {
    type Output = Self;
    fn add(self, rhs: Vector3) -> Self::Output {
        Self {
            x: self.x + rhs.get_x(),
            y: self.y + rhs.get_y(),
            z: self.z + rhs.get_z(),
        }
    }
}

impl Sub<Vector3> for Point3 {
    type Output = Self;
    fn sub(self, rhs: Vector3) -> Self::Output {
        Self {
            x: self.x - rhs.get_x(),
            y: self.y - rhs.get_y(),
            z: self.z - rhs.get_z(),
        }
    }
}
