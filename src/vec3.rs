//! Store and manipulate 3-dimensional vectors.

use rand::Rng;

#[derive(PartialEq, Clone, Copy, Debug)]
/// A 3D vector.
pub struct Vec3 {
    /// The x coordinate
    pub x: f64,
    /// The y coordinate
    pub y: f64,
    /// The z coordinate
    pub z: f64,
}

impl Vec3 {
    /// Create a new 3D vector
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    /// Create a zero-vector
    pub fn zeros() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    /// Generate a random vector in the unit cube.
    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0))
    }

    /// Generate a random vector between the given parameters.
    ///
    /// Returns a vector where the `x`, `y` and `z` components are all between `min` and `max`.
    fn random_between(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max))
    }

    /// Generate a random vector in the unit sphere.
    ///
    /// Returns a vector whose magnitude is less than 1.
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random();
            if p.length_squared() < 1.0 {
                return p
            }
        }
    }

    

    /// Calculate the dot product of two vectors.
    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Return self, but one unit in length.
    pub fn unit(&self) -> Vec3 {
        (1.0 / self.length()) * self
    }

    /// Returns the length of the vector.
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Returns the length of the vector squared.
    ///
    /// # Examples
    ///
    /// ```
    /// use raytrace::vec3::Vec3;
    ///
    /// let vector = Vec3::new(1.0, 1.0, 1.0);
    /// assert_eq!(vector.length_squared(), 3.0);
    ///
    /// let long_vector = Vec3::new(2.0, 2.0, 2.0);
    /// assert_eq!(long_vector.length_squared(), 12.0);
    /// ```
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, vector: Vec3) -> Vec3 {
        Vec3 {
            x: vector.x * self,
            y: vector.y * self,
            z: vector.z * self,
        }
    }
}

impl std::ops::Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, vector: &Vec3) -> Vec3 {
        Vec3 {
            x: vector.x * self,
            y: vector.y * self,
            z: vector.z * self,
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let x1 = 1.0;
        let y1 = -2.0;
        let z1 = 3.0;

        let x2 = 3.1394;
        let y2 = 6.2948524;
        let z2 = -0.49385;

        let vec1 = Vec3::new(x1, y1, z1);
        let vec2 = Vec3::new(x2, y2, z2);

        assert_eq!(vec1 + vec2, Vec3::new(x1 + x2, y1 + y2, z1 + z2));
    }

    #[test]
    fn test_dot() {
        let x1 = 1.0;
        let y1 = -2.0;
        let z1 = 3.0;

        let x2 = 3.1394;
        let y2 = 6.2948524;
        let z2 = -0.49385;

        let vec1 = Vec3::new(x1, y1, z1);
        let vec2 = Vec3::new(x2, y2, z2);
        let expected = (x1 * x2) + (y1 * y2) + (z1 * z2);

        assert_eq!(vec1.dot(vec2), expected);
    }

    #[test]
    fn test_dot_again() {
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 2.0
            }
            .dot(Vec3 {
                x: 2.0,
                y: 1.0,
                z: 2.0
            }),
            6.0
        );
    }

    #[test]
    fn test_dot_perpendicular() {
        let vec1 = Vec3::new(5.0, 2.0, 3.0);
        let vec2 = Vec3::new(-15.0, -6.0, 29.0);

        assert_eq!(vec1.dot(vec2), 0.0);
    }

    #[test]
    fn test_length() {
        let v = Vec3 {
            x: -2.0,
            y: -2.0,
            z: -1.0,
        };
        let u = Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        assert_eq!(v.length(), 3.0);
        assert_eq!(u.length(), 1.0);
    }

    #[test]
    fn test_length_squared() {
        let v = Vec3 {
            x: -2.0,
            y: -2.0,
            z: -1.0,
        };
        let u = Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        assert_eq!(v.length_squared(), 9.0);
        assert_eq!(u.length_squared(), 1.0);
    }

    #[test]
    fn mul() {
        assert_eq!(
            3.0 * Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            },
            Vec3 {
                x: 3.0,
                y: 6.0,
                z: 9.0
            }
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 2.0
            } - Vec3 {
                x: 2.0,
                y: 1.0,
                z: 2.0
            },
            Vec3 {
                x: -1.0,
                y: -1.0,
                z: 0.0
            }
        );
    }

    #[test]
    fn test_new_in_unit_sphere() {
        assert!(Vec3::random_in_unit_sphere().length() < 1.0);
    }
}
