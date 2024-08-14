use super::{material::Material, point::Point3, ray::Ray, vector3::Vector3};

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub point: Point3,
    pub normal: Vector3,
    pub parameter: f64,
    pub is_face_front: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        point: Point3,
        normal: Vector3,
        parameter: f64,
        is_face_front: bool,
        material: &'a dyn Material,
    ) -> Self {
        Self {
            point,
            normal,
            parameter,
            is_face_front,
            material,
        }
    }
    pub fn set_face_normal(
        ray: Ray,
        outward_normal: Vector3,
        point: Point3,
        material: &'a dyn Material,
        parameter: f64,
    ) -> Self {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length
        let is_face_front = ray.get_direction().dot_prod(outward_normal) < 0.0;
        let normal = match is_face_front {
            true => outward_normal,
            false => -(outward_normal),
        };
        Self {
            point,
            normal,
            parameter,
            is_face_front,
            material,
        }
    }
}
