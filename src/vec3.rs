use super::color::Color;
use super::math::random_f64_wide;

use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn dot(&self, v: &Vec3) -> f64 {
        (self.x * v.x) + (self.y * v.y) + (self.z * v.z)
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * v.z) - (self.z * v.y),
            y: (self.z * v.x) - (self.x * v.z),
            z: (self.x * v.y) - (self.y * v.x),
        }
    }

    pub fn length_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    /// Finds the magnitude of a vector
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Divide a vector by its length to get the vector with the same direction, but a magnitude of
    /// 1
    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

    /// Internal function, used by random_in_unit_sphere()
    fn random(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: random_f64_wide(min, max),
            y: random_f64_wide(min, max),
            z: random_f64_wide(min, max),
        }
    }

    /// Generate a random vector of less than unit length
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::random_in_unit_sphere().unit_vector()
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

/// Really???????
impl Mul for Vec3 {
    type Output = Self;

    /// Vector multiplication?? Like this???????
    fn mul(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Vec3 {
            x: self.x * (1.0 / rhs),
            y: self.y * (1.0 / rhs),
            z: self.z * (1.0 / rhs),
        }
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * (1.0 / rhs),
            y: self.y * (1.0 / rhs),
            z: self.z * (1.0 / rhs),
        }
    }
}

impl Add<Color> for Vec3 {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color {
            r: self.x + rhs.r,
            g: self.y + rhs.g,
            b: self.z + rhs.b,
        }
    }
}
