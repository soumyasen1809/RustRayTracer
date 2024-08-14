use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};

use rand::Rng;

/// The vec3 Class
///

#[derive(Clone, Copy)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn random_new() -> Self {
        Self {
            x: rand::thread_rng().r#gen::<f64>(),
            y: rand::thread_rng().r#gen::<f64>(),
            z: rand::thread_rng().r#gen::<f64>(),
        }
    }

    pub fn random_range_new(low: f64, high: f64) -> Self {
        Self {
            x: rand::thread_rng().gen_range(low, high),
            y: rand::thread_rng().gen_range(low, high),
            z: rand::thread_rng().gen_range(low, high),
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length() // needs #[derive(Clone, Copy)]
    }

    /// Returns true if the vector is very close to zero in all dimensions
    pub fn near_zero(&self) -> bool {
        let s: f64 = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn get_x(self) -> f64 {
        self.x
    }
    pub fn get_y(self) -> f64 {
        self.y
    }
    pub fn get_z(self) -> f64 {
        self.z
    }

    pub fn dot_prod(&self, rhs: Self) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    /// We can take the dot product of the surface normal and our random vector
    /// to determine if it's in the correct hemisphere.
    /// If the dot product is positive, then the vector is in the correct hemisphere.
    /// If the dot product is negative, then we need to invert the vector.
    pub fn random_on_hemisphere(&self) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        match self.dot_prod(on_unit_sphere) > 0.0 {
            true => on_unit_sphere,
            false => -on_unit_sphere,
        }
    }

    /// With a random vector in the unit sphere we need to normalize it to
    /// get a vector on the unit sphere
    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    /// We will use a rejection method to generate the random vector inside
    /// of the unit sphere. Pick a random point in the unit cube, where x, y, z
    /// all range from −1 to +1, and reject this point if it is outside the unit sphere.
    fn random_in_unit_sphere() -> Self {
        loop {
            // use loop instead of while(true)
            let random_vec: Vector3 = Vector3::random_range_new(-1.0, 1.0);
            if random_vec.length_squared() < 1.0 {
                break random_vec; // to return from loop
            }
        }
    }

    pub fn reflection(&self, normal_vec: &Self) -> Self {
        return *self - ((*normal_vec * (self.dot_prod(*normal_vec))) * 2.0);
    }

    pub fn refraction(&self, normal_vec: &Self, ratio_refractive_index: f64) -> Self {
        let cos_theta: f64 = (-self.dot_prod(*normal_vec)).min(1.0); // std::fmin
        let ray_out_perp: Vector3 = (*self + (*normal_vec * cos_theta)) * ratio_refractive_index;
        let ray_out_par: Vector3 =
            *normal_vec * (-(((1.0 - ray_out_perp.length_squared()).abs()).sqrt()));

        ray_out_par + ray_out_perp
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

// https://doc.rust-lang.org/std/ops/trait.Neg.html
impl Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<f64> for Vector3 {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl MulAssign for Vector3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

// https://doc.rust-lang.org/std/ops/trait.Mul.html
impl Mul<f64> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul for Vector3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

pub trait Cross {
    fn cross_prod(&self, rhs: Self) -> Self;
}

impl Cross for Vector3 {
    fn cross_prod(&self, rhs: Self) -> Self {
        Self {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }
}
