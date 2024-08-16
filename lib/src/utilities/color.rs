use std::io::Write;
use std::iter::Sum;
use std::ops::Add;
use std::{
    fs::File,
    ops::{AddAssign, Div, Mul, MulAssign},
};

/// The color Class
///

#[derive(Clone, Copy)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color { red, green, blue }
    }

    pub fn get_r(self) -> f64 {
        self.red
    }
    pub fn get_g(self) -> f64 {
        self.green
    }
    pub fn get_b(self) -> f64 {
        self.blue
    }

    pub fn write_color(&mut self, file: &mut File) -> std::io::Result<()> {
        // Apply a linear to gamma transform for gamma
        self.red = Self::linear_to_gamma_transformation(self.red);
        self.green = Self::linear_to_gamma_transformation(self.green);
        self.blue = Self::linear_to_gamma_transformation(self.blue);

        // Translate the [0,1] component values to the byte range [0,255].
        let rbyte = (256.0 * self.red.clamp(0.0, 0.999)) as i32;
        let gbyte = (256.0 * self.green.clamp(0.0, 0.999)) as i32;
        let bbyte = (256.0 * self.blue.clamp(0.0, 0.999)) as i32;

        writeln!(file, "{} {} {}", rbyte, gbyte, bbyte)
    }

    /// Transform our data into gamma space so that an image viewer
    /// can more accurately display our image
    fn linear_to_gamma_transformation(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            return linear_component.sqrt();
        }
        return 0.0;
    }
}

impl Default for Color {
    fn default() -> Self {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        // iter.fold() is a method in Rust that allows you to
        // accumulate or reduce the elements of an iterator into a single value.
        // init: The initial value of the accumulator, which is passed to
        // the closure as the first argument in the first iteration.
        // E.g.: Summing a list: numbers.iter().fold(0, |a, &b| a + b);
        iter.fold(Color::default(), |a, b| a + b)
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.red *= rhs.red;
        self.green *= rhs.green;
        self.blue *= rhs.blue;
    }
}

// https://doc.rust-lang.org/std/ops/trait.Mul.html
impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl Div<f64> for Color {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            red: self.red / rhs,
            green: self.green / rhs,
            blue: self.blue / rhs,
        }
    }
}

pub trait Cross {
    fn cross_prod(&self, rhs: Self) -> Self;
}

impl Cross for Color {
    fn cross_prod(&self, rhs: Self) -> Self {
        Self {
            red: (self.green * rhs.blue) - (self.blue * rhs.green),
            green: (self.blue * rhs.red) - (self.red * rhs.blue),
            blue: (self.red * rhs.green) - (self.green * rhs.red),
        }
    }
}
