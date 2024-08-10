use super::{point::Point3, ray::Ray, vector3::Vector3};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vector3,
    pub parameter: f64,
    pub is_face_front: bool,
}

impl HitRecord {
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
