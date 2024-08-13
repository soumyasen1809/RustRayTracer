use super::{point::Point3, vector3::Vector3};

/// The Ray Class
///

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, parameter: f64) -> Point3 {
        let positon_fn = |t: f64| self.origin + (self.direction * t);
        positon_fn(parameter)
    }

    pub fn get_origin(&self) -> Point3 {
        self.origin
    }
    pub fn get_direction(&self) -> Vector3 {
        self.direction
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Point3::default(),
            direction: Vector3::default(),
        }
    }
}
