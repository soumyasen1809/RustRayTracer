use super::{point::Point3, ray::Ray, vector3::Vector3};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vector3,
    pub parameter: f64,
    pub is_face_front: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            point: Point3::default(),
            normal: Vector3::default(),
            parameter: f64::default(),
            is_face_front: bool::default(),
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length
        self.is_face_front = ray.get_direction().dot_prod(*outward_normal) < 0.0;
        self.normal = match self.is_face_front {
            true => *outward_normal,
            false => -(*outward_normal),
        }
    }
}
