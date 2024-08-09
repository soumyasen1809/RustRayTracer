use std::io::Write;
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

    pub fn write_color(&self, file: &mut File) -> std::io::Result<()> {
        let rbyte = (255.999 * self.red) as i32;
        let gbyte = (255.999 * self.green) as i32;
        let bbyte = (255.999 * self.blue) as i32;

        writeln!(file, "{} {} {}", rbyte, gbyte, bbyte)
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
