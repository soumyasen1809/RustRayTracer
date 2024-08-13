use super::{color::Color, hit_record::HitRecord, ray::Ray, vector3::Vector3};

// https://github.com/ebkalderon/ray-tracing-in-one-weekend/commits/master/?before=afc5b8807ba4a342b09c83361968e7ddc284fc12+70
pub trait Material {
    fn scatter(
        &self,
        incoming_ray: Ray,
        record: HitRecord,
        attenuation: Color,
        scattered_ray: Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: &Color) -> Self {
        Self { albedo: *albedo }
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Self {
            albedo: Color::default(),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _incoming_ray: Ray,
        record: HitRecord,
        mut _attenuation: Color,
        mut _scattered_ray: Ray,
    ) -> bool {
        // Lambertian scatter
        let mut scatter_direction: Vector3 = record.normal + Vector3::random_unit_vector();

        // Catch the near-zero scatter directions
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        _scattered_ray = Ray::new(record.point, scatter_direction);
        _attenuation = self.albedo;

        true
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: &Color) -> Self {
        Self { albedo: *albedo }
    }
}

impl Default for Metal {
    fn default() -> Self {
        Self {
            albedo: Color::default(),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        incoming_ray: Ray,
        record: HitRecord,
        mut _attenuation: Color,
        mut _scattered_ray: Ray,
    ) -> bool {
        // Metal material with reflectance function
        let reflect_direction: Vector3 = incoming_ray.get_direction().reflection(&record.normal);
        _scattered_ray = Ray::new(record.point, reflect_direction);
        _attenuation = self.albedo;

        true
    }
}
