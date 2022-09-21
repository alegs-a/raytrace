//! Defines vectors and points, and their related operations

/// A 3-dimensional vector.
#[derive(Copy, Clone, Debug)]
pub struct Vec3d {
    /// The x component of the vector
    pub x: f64,
    /// The y component of the vector
    pub y: f64,
    /// The z component of the vector
    pub z: f64,
}

impl Vec3d {
    /// Initialises a new instance of Vec3d with unit length components (i.e. (1, 1, 1)).
    pub fn new() -> Vec3d {
        let x: f64 = 1.0;
        let y: f64 = 1.0;
        let z: f64 = 1.0;

        Vec3d { x, y, z }
    }

    /// Returns the vector's length squared
    /// Calculated as (a^2 + b^2 + c^2)
    /// # Examples
    /// ```
    /// use raytrace::math::*;
    /// assert_eq!(Vec3d::new().length_squared(), 3.0)
    /// ```
    pub fn length_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    /// Calculates the length of the vector
    /// Takes the square root of the length_squared
    /// # Panics
    /// - Uses `.sqrt()`, which panics if it receives a negative value other than -0.0
    /// # Examples
    /// ```
    /// use raytrace::math::*;
    /// let vector = Vec3d::new();
    /// let vector_length = vector.length();
    /// ```
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(self) -> Vec3d {
        self / self.length()
    }

    /// Scalar multiplication of a vector
    pub fn scalarp(&self, s: f64) -> Vec3d {
        Vec3d {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }

    /// Dot product of two vectors
    pub fn dotp(&self, rhs: &Vec3d) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    /// Cross product of two vectors
    pub fn crossp(&self, rhs: &Vec3d) -> Vec3d {
        Vec3d {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }
}

/// Normalises a vector.
/// Returns vector with same direction but a magnitude of 1.
pub fn normalise(v: Vec3d) -> Vec3d {
    v.scalarp(1.0 / v.length())
}

impl std::ops::Add for Vec3d {
    type Output = Vec3d;

    fn add(self, rhs: Self) -> Vec3d {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vec3d {
    type Output = Vec3d;

    fn sub(self, rhs: Self) -> Vec3d {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Div<f64> for Vec3d {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        self * (1.0 / rhs)
    }
}

impl std::ops::Mul<f64> for Vec3d {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vec3d {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::fmt::Display for Vec3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector: {}, {}, {}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
#[test]
fn test_length_squared() {
    let vec = Vec3d::new();
    assert_eq!(vec.length_squared(), 3.0);
}

#[test]
fn test_length() {
    let vec = Vec3d::new();
    assert_eq!(vec.length(), 1.7320508075688772);

    let complex_vec = Vec3d {
        x: 1.0,
        y: -1.0,
        z: 1.0,
    };
    assert_eq!(complex_vec.length(), 1.7320508075688772);
}
