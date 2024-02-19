use std::ops::{Add, Mul, Sub};
/// A 3D vector
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    #[allow(dead_code)]
    /// Generates a new Vec3
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    /// Convenience function!!! Do not use in actual code!!!!!!!!!!
    pub fn new_from_i32s(x: i32, y: i32, z: i32) -> Self {
        Vec3 {
            x: x as f64,
            y: y as f64,
            z: z as f64,
        }
    }

    /// Calculates the length of the vector and squares it.
    ///
    /// In fact, this is the intermediate step to computing the non-squared length.
    fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Calculates the length of the vector.
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Calculates the dot product of two vectors.
    pub fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y * self.z * rhs.z
    }

    /// Computes the cross product of two vectors.
    ///
    /// The cross product is the vector that is perpendicular to both input vectors. The cross
    /// product is degenerate (zero) if the two input vectors are parallel.
    ///
    /// # Examples
    /// ```
    /// let vec1 = Vec3::new(1.0, 0.0, 0.0);
    /// let vec2 = Vec3::new(0.0, 1.0, 0.0);
    /// 
    /// let cross = vec1.cross(vec2);
    ///
    /// assert_eq!(cross, Vec3::new(0.0, 0.0, 1.0));
    /// ```
    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// for some vector v, allow 2 * v but not v * 2 or v * v
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}
