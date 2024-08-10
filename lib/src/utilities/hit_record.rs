use super::{point::Point3, vector3::Vector3};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vector3,
    pub parameter: f64,
}
