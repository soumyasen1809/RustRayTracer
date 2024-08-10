use super::{hit_record::HitRecord, point::Point3, ray::Ray, vector3::Vector3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius: radius.max(0.0), // (std::fmax(0,radius)
        }
    }

    pub fn get_center(&self) -> Point3 {
        self.center
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let dist_center_origin: Vector3 = (self.center - ray.get_origin()).as_vec();
        let a: f64 = ray.get_direction().length_squared();
        let h: f64 = ray.get_direction().dot_prod(dist_center_origin);
        let c: f64 = (dist_center_origin.length_squared()) - (self.radius * self.radius);

        let discriminant: f64 = (h * h) - (a * c);
        if discriminant < 0.0 {
            return false;
        }

        // Find the nearest root that lies in the acceptable range.
        let sqrt_discriminant: f64 = discriminant.sqrt();
        let mut root: f64 = (h - sqrt_discriminant) / a;
        if (root <= t_min) || (root >= t_max) {
            root = (h + sqrt_discriminant) / a;
            if (root <= t_min) || (root >= t_max) {
                return false;
            }
        }

        record.parameter = root;
        record.point = ray.position(record.parameter);
        let outward_normal = (record.point - self.center).as_vec() / self.radius;
        record.set_face_normal(ray, &outward_normal);

        true
    }
}
