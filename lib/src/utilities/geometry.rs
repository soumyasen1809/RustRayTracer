use super::{
    hit_record::HitRecord, interval::Interval, material::Material, point::Point3, ray::Ray,
    vector3::Vector3,
};

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_interval: Interval, record: HitRecord) -> bool;
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
    fn hit<'a>(&'a self, ray: &Ray, ray_interval: Interval, mut record: HitRecord<'a>) -> bool {
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
        if !ray_interval.contains(root) {
            root = (h + sqrt_discriminant) / a;
            if !ray_interval.contains(root) {
                return false;
            }
        }

        record.parameter = root;
        record.point = ray.position(record.parameter);
        let outward_normal = (record.point - self.center).as_vec() / self.radius;
        record.set_face_normal(ray, &outward_normal);
        record.material = &*self.material;

        true
    }
}

pub struct HittableObjects {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableObjects {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, new_object: Box<dyn Hittable>) {
        self.objects.push(new_object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableObjects {
    fn hit(&self, ray: &Ray, ray_interval: Interval, mut record: HitRecord) -> bool {
        let t_min: f64 = ray_interval.min;
        let t_max: f64 = ray_interval.max;
        let temp_record: HitRecord = record; // needed since to mut this, we need to initialize it
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;

        for object in self.objects.iter() {
            if object.hit(ray, Interval::new(t_min, closest_so_far), temp_record) {
                hit_anything = true;
                closest_so_far = record.parameter;
                record = temp_record;
            }
        }

        hit_anything
    }
}
