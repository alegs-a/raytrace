use std::ops::{Add, AddAssign, Div, Mul, Sub};

/// Define an RGB color
#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    fn clamp(x: f64, min: f64, max: f64) -> f64 {
        if x < min {
            return min;
        }
        if x > max {
            return max;
        }
        x
    }

    pub fn write_color(color: Color, samples_per_pixel: u32) {
        let mut r = color.r;
        let mut g = color.g;
        let mut b = color.b;

        // Divide the colour by the number of samples per pixel and gamma-correct for gamma = 2.0
        let scale = 1.0 / samples_per_pixel as f64;
        r = (r * scale).sqrt();
        g = (g * scale).sqrt();
        b = (b * scale).sqrt();

        // Write the translated [0, 255] value of each colour component
        println!(
            "{} {} {}",
            (256.0 * Self::clamp(r, 0.0, 0.999)) as i32,
            (256.0 * Self::clamp(g, 0.0, 0.999)) as i32,
            (256.0 * Self::clamp(b, 0.0, 0.999)) as i32,
        );
    }

    pub fn white() -> Color {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    pub fn black() -> Color {
        Color {
            r: 0.0,
            b: 0.0,
            g: 0.0,
        }
    }

    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
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

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
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

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
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
