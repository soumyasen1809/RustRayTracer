use super::{color::Color, point::Point3, vector3::Vector3};

/// The Ray Class
///

pub struct Ray {
    origin: Point3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, parameter: f64) -> Point3 {
        let positon_fn = |t: &f64| self.origin + (self.direction * *t);
        positon_fn(&parameter)
    }

    pub fn get_origin(&self) -> Point3 {
        self.origin
    }
    pub fn get_direction(&self) -> Vector3 {
        self.direction
    }

    /// Implements a simple gradient
    pub fn ray_color(&self) -> Color {
        let unit_direction: Vector3 = self.get_direction().unit_vector();
        let a: f64 = 0.5 * (unit_direction.get_y() + 1.0);

        // Linear blending or Linear interpolation
        // blendedValue = (1−a)⋅startValue+a⋅endValue
        (Color::new(1.0, 1.0, 1.0) * (1.0 - a)) + (Color::new(0.5, 0.7, 1.0) * a)
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
