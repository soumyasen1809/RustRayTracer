use super::{point::Point, vector3::Vector3};

/// The Ray Class
///

pub struct Ray {
    origin: Point,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, parameter: f64) -> Point {
        let positon_fn = |t: &f64| self.origin + (self.direction * *t);
        positon_fn(&parameter)
    }

    pub fn get_origin(&self) -> Point {
        self.origin
    }
    pub fn get_direction(&self) -> Vector3 {
        self.direction
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Point::default(),
            direction: Vector3::default(),
        }
    }
}
