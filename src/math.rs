//! Contains the maths functionality needed for Raytrace.

///
/// A 3-dimensional vector and related operations.
pub struct Vec3d {
    /// The x component of the vector
    pub x: f64,
    /// The y component of the vector
    pub y: f64,
    /// The z component of the vector
    pub z: f64
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
    /// Takes the square root of the [length_squared]
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
    
    /// Scalar multiplication of a vector
    fn scalarp(&self, s: f64) -> Vec3d {
        Vec3d {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }

    /// Dot product of two vectors
    fn dotp(&self, rhs: &Vec3d) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    /// Cross product of two vectors
    fn crossp(&self, rhs: &Vec3d) -> Vec3d {
        Vec3d {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }
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

impl Vec3d {
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

