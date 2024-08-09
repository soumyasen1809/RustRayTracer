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

    /// Implements a simple gradient
    pub fn ray_color(&self) -> Color {
        // Check if sphere is hit: color it red
        let sphere_hit = is_sphere_hit(&Point3::new(0.0, 0.0, -1.0), 0.5, &self);
        if sphere_hit {
            println!("Inside sphere hit");
            return Color::new(1.0, 0.0, 0.0);
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

fn is_sphere_hit(center: &Point3, radius: f64, ray: &Ray) -> bool {
    let dist_center_origin: Vector3 = (*center - ray.get_origin()).as_vec();
    let a: f64 = ray.get_direction().dot_prod(ray.get_direction());
    let b: f64 = -2.0 * (ray.get_direction().dot_prod(dist_center_origin));
    let c: f64 = (dist_center_origin.dot_prod(dist_center_origin)) - (radius * radius);

    let discriminant: f64 = (b * b) - (4.0 * (a * c)); // For finding roots of a quadratic eqn b^2 -4ac = 0
    discriminant >= 0.0
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Point3::default(),
            direction: Vector3::default(),
        }
    }
}
