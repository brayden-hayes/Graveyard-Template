use std::ops::Mul;

#[derive(Clone, Copy,)]
pub struct Rotation2D {
    cos: f64,
    sin: f64,
}

impl Rotation2D {
    pub fn zero() -> Self {
        Self {
            cos: 1.0,
            sin: 0.0,
        }
    }

    pub fn from_radians(radians: f64) -> Self {
        Self {
            cos: radians.cos(),
            sin: radians.sin(),
        }
    }

    pub fn radians(&self) -> f64 {
        self.sin.atan2(self.cos)
    }

    pub fn inverse(&self) -> Self {
        Self {
            cos: self.cos,
            sin: -self.sin,
        }
    }

    pub fn cos(&self) -> f64 {
        self.cos
    }

    pub fn sin(&self) -> f64 {
        self.sin
    }
}

impl Mul for Rotation2D {
    type Output = Rotation2D;

    fn mul(self, rhs: Rotation2D) -> Rotation2D {
        Rotation2D {
            cos: self.cos * rhs.cos - self.sin * rhs.sin,
            sin: self.sin * rhs.cos + self.cos * rhs.sin,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    use crate::utils::math::clamp_radians;

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
        assert!((rot3.radians() - clamp_radians(7.0 * PI / 6.0)).abs() < 1e-10);
    }

    #[test]
    fn test_zero_rotation() {
        let rot = Rotation2D::zero();

        assert!((rot.cos() - 1.0).abs() < 1e-10);
        assert!(rot.sin().abs() < 1e-10);
    }
}