use super::{color::Color, hit_record::HitRecord, ray::Ray, vector3::Vector3};

#[derive(Clone)]
pub struct Scatter {
    pub scattered_ray: Ray,
    pub attenuation: Color,
}

// https://github.com/ebkalderon/ray-tracing-in-one-weekend/commits/master/?before=afc5b8807ba4a342b09c83361968e7ddc284fc12+70
pub trait Material: Send + Sync {
    fn scatter(&self, incoming_ray: Ray, record: &HitRecord) -> Option<Scatter>;
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
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

#[derive(Clone)]
pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }

    pub fn reflectance(cos_theta: f64, r_index: f64) -> f64 {
        let mut r0: f64 = (1.0 - r_index) / (1.0 + r_index);
        r0 = r0 * r0;
        r0 + ((1.0 - r0) * ((1.0 - cos_theta).powi(5)))
    }
}

impl Default for Dielectric {
    fn default() -> Self {
        Self {
            refractive_index: 1.0,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, incoming_ray: Ray, record: &HitRecord) -> Option<Scatter> {
        let attenuation: Color = Color::new(1.0, 1.0, 1.0);
        let r_index: f64 = if record.is_face_front {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let unit_direction: Vector3 = incoming_ray.get_direction().unit_vector();

        let cos_theta: f64 = (-unit_direction.dot_prod(record.normal)).min(1.0); // std::fmin
        let sin_theta: f64 = (1.0 - (cos_theta * cos_theta)).sqrt();
        let can_refract: bool = r_index * sin_theta <= 1.0;

        let ray_direction: Vector3 = if can_refract {
            unit_direction.refraction(&record.normal, r_index)
        } else {
            unit_direction.reflection(&record.normal)
        };
        let scattered_ray: Ray = Ray::new(record.point, ray_direction);

        Some(Scatter {
            scattered_ray,
            attenuation,
        })
    }
}
