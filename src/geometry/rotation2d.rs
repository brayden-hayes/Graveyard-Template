use std::ops::Mul;

/// A 2D rotation, represented by its cosine and sine values to prevent gimbal lock and ensure numerical stability.
/// It is the main representation of a rotation in 2D space, and can be used to rotate translations and poses.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rotation2D {
    cos: f64,
    sin: f64,
}

impl Rotation2D {

    /// Creates a new Rotation2D pointing in the zero direction (cos = 1, sin = 0).
    pub fn zero() -> Self {
        Self {
            cos: 1.0,
            sin: 0.0,
        }
    }

    /// Creates a new Rotation2D from the given angle in radians.
    pub fn from_radians(radians: f64) -> Self {
        Self {
            cos: radians.cos(),
            sin: radians.sin(),
        }
    }

    /// Returns the angle of the rotation in radians.
    pub fn radians(&self) -> f64 {
        self.sin.atan2(self.cos)
    }

    /// Returns the inverse of the rotation, which is equivalent to rotating in the opposite direction.
    pub fn inverse(&self) -> Self {
        Self {
            cos: self.cos,
            sin: -self.sin,
        }
    }

    /// Getter for the cosine of the rotation angle.
    pub fn cos(&self) -> f64 {
        self.cos
    }

    /// Getter for the sine of the rotation angle.
    pub fn sin(&self) -> f64 {
        self.sin
    }
}

/// Implement the multiplication operator for Rotation2D, allowing for the composition of two rotations.
/// This is equivalent to applying the first rotation followed by the second rotation.
impl Mul for Rotation2D {
    type Output = Rotation2D;

    fn mul(self, rhs: Rotation2D) -> Rotation2D {
        Rotation2D {
            cos: self.cos * rhs.cos - self.sin * rhs.sin,
            sin: self.sin * rhs.cos + self.cos * rhs.sin,
        }
    }
}



/// Unit tests for the Rotation2D struct and its methods.
#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    use crate::utils::math::wrap_radians;

    #[test]
    fn test_radians() {
        let rot = Rotation2D::from_radians(PI / 2.0);
        assert!((rot.radians() - (PI / 2.0)).abs() < 1e-10);
    }

    #[test]
    fn test_rot_mul() {
        let rot1 = Rotation2D::from_radians(PI / 2.0);
        let rot2 = Rotation2D::from_radians(2.0 * PI / 3.0);
        let rot3 = rot1*rot2;
        assert!((rot3.radians() - wrap_radians(7.0 * PI / 6.0)).abs() < 1e-10);
    }

    #[test]
    fn test_zero_rotation() {
        let rot = Rotation2D::zero();

        assert!((rot.cos() - 1.0).abs() < 1e-10);
        assert!(rot.sin().abs() < 1e-10);
    }
}