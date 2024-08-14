use super::{
    hit_record::HitRecord, interval::Interval, material::Material, point::Point3, ray::Ray,
    vector3::Vector3,
};

pub trait Hittable {
    fn hit(&self, ray: Ray, ray_interval: Interval) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Box<dyn Material>) -> Self {
        Self {
            center,
            radius: radius.max(0.0), // std::fmax(0,radius)
            material,
        }
    }

    pub fn get_center(&self) -> Point3 {
        self.center
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_interval: Interval) -> Option<HitRecord> {
        let dist_center_origin: Vector3 = (self.center - ray.get_origin()).as_vec();
        let a: f64 = ray.get_direction().length_squared();
        let h: f64 = ray.get_direction().dot_prod(dist_center_origin);
        let c: f64 = (dist_center_origin.length_squared()) - (self.radius * self.radius);

        let discriminant: f64 = (h * h) - (a * c);
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the acceptable range.
        let sqrt_discriminant: f64 = discriminant.sqrt();
        let mut root: f64 = (h - sqrt_discriminant) / a;
        if !ray_interval.contains(root) {
            root = (h + sqrt_discriminant) / a;
            if !ray_interval.contains(root) {
                return None;
            }
        }

        let parameter = root;
        let point = ray.position(root);
        let outward_normal = (point - self.center).as_vec() / self.radius;
        let material = &*self.material;

        Some(HitRecord::set_face_normal(
            ray,
            outward_normal,
            point,
            material,
            parameter,
        ))
    }
}

impl<T> Hittable for T
where
    T: AsRef<[Box<dyn Hittable>]>,
{
    fn hit(&self, ray: Ray, ray_interval: Interval) -> Option<HitRecord> {
        let t_min: f64 = ray_interval.min;
        let t_max: f64 = ray_interval.max;

        let mut closest_so_far: Option<HitRecord> = None;

        for object in self.as_ref().iter() {
            let t_max_local = closest_so_far
                .as_ref()
                .map(|hit| hit.parameter)
                .unwrap_or(t_max);
            if let Some(record) = object.hit(ray, Interval::new(t_min, t_max_local)) {
                closest_so_far = Some(record);
            }
        }

        closest_so_far
    }
}
