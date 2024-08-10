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
        let positon_fn = |t: f64| self.origin + (self.direction * t);
        positon_fn(parameter)
    }

    pub fn get_origin(&self) -> Point3 {
        self.origin
    }
    pub fn get_direction(&self) -> Vector3 {
        self.direction
    }

    fn sphere_hit(&self, center: &Point3, radius: f64) -> f64 {
        let dist_center_origin: Vector3 = (*center - self.get_origin()).as_vec();
        let a: f64 = self.get_direction().dot_prod(self.get_direction());
        let b: f64 = -2.0 * (self.get_direction().dot_prod(dist_center_origin));
        let c: f64 = (dist_center_origin.dot_prod(dist_center_origin)) - (radius * radius);

        let discriminant: f64 = (b * b) - (4.0 * (a * c)); // For finding roots of a quadratic eqn b^2 -4ac = 0
        if discriminant >= 0.0 {
            return (-b - discriminant.sqrt()) / (2.0 * a);
        } else {
            return -1.0;
        }
    }

    /// Implements a simple gradient
    pub fn ray_color(&self) -> Color {
        // Check if sphere is hit: color it red
        let sphere_hit_color = self.sphere_hit(&Point3::new(0.0, 0.0, -1.0), 0.5);
        if sphere_hit_color > 0.0 {
            let normal_vec: Vector3 = (self.position(sphere_hit_color).as_vec()
                - Vector3::new(0.0, 0.0, -1.0))
            .unit_vector();
            return (Color::new(
                normal_vec.get_x() + 1.0,
                normal_vec.get_y() + 1.0,
                normal_vec.get_z() + 1.0,
            )) * 0.5;
        }

        // Color the background blue
        let unit_direction: Vector3 = self.get_direction().unit_vector();
        let a: f64 = 0.5 * (unit_direction.get_y() + 1.0);

        // Linear blending or Linear interpolation
        // blendedValue = (1−a)⋅startValue+a⋅endValue
        // Returns a blue blended color
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
