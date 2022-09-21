// use std::fs::File;

/// An image which can be written out using the .ppm format
pub struct Image {
    pub width: i32,
    pub height: i32,
    pub data: String,
}

impl Image {
    /// Write the data in an `Image` to stdout (eventually a file).
    /// # Note
    /// `_filename` is currently to supress warning about
    /// unused variable, will return to `filename:` when
    /// output to file functionality is added.
    pub fn print(&self, _filename: &str) {
        println!("P3");
        println!("{} {}", self.width, self.height);
        println!("255");
        println!("{}", self.data)
    }

    /// Create an `Image` with a generic rainbow for testing purposes
    pub fn test(width: i32, height: i32) -> Image {
        let mut data: String = String::from("");

        for y in (0..height).rev() {
            for x in 0..width {
                let colour = Colour {
                    r: x as f64 / (width - 1) as f64,
                    g: y as f64 / (height - 1) as f64,
                    b: 0.25,
                };
                data.push_str(&colour.write());
            }
        }

        Image {
            width,
            height,
            data,
        }
    }
}

/// The colour of a pixel
#[derive(Copy, Clone, Debug)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    /// Prints to stdout the pixel's colour as values between 0 and 255, with each value
    /// seperated by a space.
    /// Assumes each of `r`, `g` and `b` contain values between 0 and 1.
    pub fn write(&self) -> String {
        format!(
            "{} {} {}",
            ((255.999 * self.r) as i32),
            ((255.999 * self.g) as i32),
            ((255.999 * self.b) as i32),
        )
    }
}

impl std::ops::Mul<f64> for Colour {
    type Output = Colour;

    fn mul(self, rhs: f64) -> Colour {
        Colour {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl std::ops::Add for Colour {
    type Output = Colour;

    fn add(self, rhs: Colour) -> Colour {
        Colour {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Colour: {}, {}, {}", self.r, self.g, self.b)
    }
}
