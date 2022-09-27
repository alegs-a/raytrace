use std::ops::{Add, Div, Mul, Sub};

/// Define an RGB color
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn write_color(color: Color) {
        println!(
            "{} {} {}",
            (255.999 * color.r) as i32,
            (255.999 * color.g) as i32,
            (255.999 * color.b) as i32,
        );
    }

    pub fn white() -> Color {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {
            r,
            g,
            b,
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Add for &Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Color {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl Sub for &Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Color {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<f64> for &Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Color {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Color {
            r: self.r * (1.0 / rhs),
            g: self.g * (1.0 / rhs),
            b: self.b * (1.0 / rhs),
        }
    }
}

impl Div<f64> for &Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Color {
        Color {
            r: self.r * (1.0 / rhs),
            g: self.g * (1.0 / rhs),
            b: self.b * (1.0 / rhs),
        }
    }
}
