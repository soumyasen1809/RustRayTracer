use super::{
    color::Color, hit_record::HitRecord, point::Point3, sphere::Hittable, vector3::Vector3,
};

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

    pub fn ray_color(&self, world: &dyn Hittable) -> Color {
        let mut record: HitRecord = HitRecord::new(); // needed since to mut this, we need to initialize it
        if world.hit(&self, 0.0, std::f64::INFINITY, &mut record) {
            return (Color::new(
                record.normal.get_x(),
                record.normal.get_y(),
                record.normal.get_z(),
            ) + Color::new(1.0, 1.0, 1.0))
                * 0.5;
        }

        // Color the background blue - Implements a simple gradient
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
