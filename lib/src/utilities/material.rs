use super::{color::Color, hit_record::HitRecord, ray::Ray, vector3::Vector3};

#[derive(Clone)]
pub struct Scatter {
    pub scattered_ray: Ray,
    pub attenuation: Color,
}

// https://github.com/ebkalderon/ray-tracing-in-one-weekend/commits/master/?before=afc5b8807ba4a342b09c83361968e7ddc284fc12+70
pub trait Material {
    fn scatter(&self, incoming_ray: Ray, record: &HitRecord) -> Option<Scatter>;
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Self {
            albedo: Color::new(0.5, 0.5, 0.5),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _incoming_ray: Ray, record: &HitRecord) -> Option<Scatter> {
        // Lambertian scatter
        let mut scatter_direction: Vector3 = record.normal + Vector3::random_unit_vector();

        // Catch the near-zero scatter directions
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        Some(Scatter {
            scattered_ray: Ray::new(record.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Default for Metal {
    fn default() -> Self {
        Self {
            albedo: Color::new(0.8, 0.8, 0.8),
            fuzz: 1.0,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, incoming_ray: Ray, record: &HitRecord) -> Option<Scatter> {
        // Metal material with reflectance function
        let reflect_direction: Vector3 = (incoming_ray
            .get_direction()
            .unit_vector()
            .reflection(&record.normal))
            + (Vector3::random_unit_vector() * self.fuzz);
        let scattered_ray = Ray::new(record.point, reflect_direction);
        let attenuation = self.albedo;
        if scattered_ray.get_direction().dot_prod(record.normal) > 0.0 {
            return Some(Scatter {
                scattered_ray,
                attenuation,
            });
        }
        None
    }
}
