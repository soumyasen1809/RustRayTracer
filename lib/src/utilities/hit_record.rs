use std::rc::Rc;

use super::{material::Material, point::Point3, ray::Ray, vector3::Vector3};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vector3,
    pub parameter: f64,
    pub is_face_front: bool,
    pub material: Option<Rc<dyn Material>>,
}

impl HitRecord {
    pub fn new(
        point: Point3,
        normal: Vector3,
        parameter: f64,
        is_face_front: bool,
        material: Option<Rc<dyn Material>>,
    ) -> Self {
        Self {
            point,
            normal,
            parameter,
            is_face_front,
            material,
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length
        self.is_face_front = ray.get_direction().dot_prod(outward_normal) < 0.0;
        self.normal = match self.is_face_front {
            true => outward_normal,
            false => -(outward_normal),
        }
    }

    pub fn set_record(&mut self, other_record: &HitRecord) {
        self.point = other_record.point.clone();
        self.normal = other_record.normal.clone();
        self.parameter = other_record.parameter.clone();
        self.is_face_front = other_record.is_face_front.clone();
        self.material = other_record.material.clone();
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            material: None,
            point: Point3::default(),
            normal: Vector3::default(),
            parameter: 0.0,
            is_face_front: false,
        }
    }
}
