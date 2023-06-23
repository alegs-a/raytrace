//! Store and manipulate colour data for a pixel
use std::io;
use std::io::prelude::*;

/// The RBG colour of a pixel.
#[derive(PartialEq, Debug)]
pub struct Colour {
    /// The red component
    pub r: f64,
    /// The green component
    pub g: f64,
    /// The blue component
    pub b: f64,
}

impl Colour {
    /// Create a new Colour with the given values
    pub fn new(r: f64, g: f64, b: f64) -> Colour {
        Colour { r, g, b }
    }

    /// The colour red.
    ///
    /// Mostly used for development purposes, i.e. to easily see which pixels meet a criterion.
    pub fn red() -> Colour {
        Colour::new(1.0, 0.0, 0.0)
    }

    /// The colour black.
    pub fn black() -> Colour {
        Colour::new(0.0, 0.0, 0.0)
    }

    /// Convert the RGB values of the `Colour` to the \[0,255\] values to be written to the output.
    ///
    /// Takes the sum of the colours from `samples_per_pixel` rays, and scales them to be within
    /// \[0,1\].
    pub fn write<W: Write>(self, writer: &mut io::BufWriter<W>, samples_per_pixel: i32) -> io::Result<()> {
        let scale = 1.0 / samples_per_pixel as f64;
        let r = ((scale * self.r).clamp(0.0, 1.0) * 255.0) as i32;
        let g = ((scale * self.g).clamp(0.0, 1.0) * 255.0) as i32;
        let b = ((scale * self.b).clamp(0.0, 1.0) * 255.0) as i32;
        writeln!(writer, "{} {} {}", r, g, b)?;
        Ok(())
    }
}

impl std::ops::Add for Colour {
    type Output = Colour;
    fn add(self, other: Colour) -> Colour {
        Colour {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl std::ops::AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::Sub for Colour {
    type Output = Colour;
    fn sub(self, other: Colour) -> Colour {
        Colour {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl std::ops::Mul<Colour> for f64 {
    type Output = Colour;
    fn mul(self, colour: Colour) -> Colour {
        Colour {
            r: colour.r * self,
            g: colour.g * self,
            b: colour.b * self,
        }
    }
}
