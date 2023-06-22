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

    /// Convert the \[0,1\] values of the `Colour` to the \[0,255\] values to be written to the output.
    pub fn write<W: Write>(self, writer: &mut io::BufWriter<W>) -> io::Result<()> {
        let r = (self.r * 255.0) as i32;
        let g = (self.g * 255.0) as i32;
        let b = (self.b * 255.0) as i32;
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