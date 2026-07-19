use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Neg;

use super::rotation2d::Rotation2D;

#[derive(Clone, Copy)]
pub struct Translation2D {
    x: f64,
    y: f64,
}

impl Translation2D {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }

     pub fn new(x: f64, y: f64,) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn from_polar(r: f64, theta: Rotation2D) -> Self {
        Self {
            x: r * theta.cos(),
            y: r * theta.sin(),
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn distance(&self, other: Translation2D) -> f64 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }
}

impl Add for Translation2D {
    type Output = Translation2D;

    fn add(self, rhs: Translation2D) -> Translation2D {
        Translation2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Translation2D {
    type Output = Translation2D;

    fn sub(self, rhs: Translation2D) -> Translation2D {
        Translation2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<Rotation2D> for Translation2D {
    type Output = Translation2D;

    fn mul(self, rhs: Rotation2D) -> Translation2D {
        Translation2D {
            x: self.x*rhs.cos() - self.y*rhs.sin(),
            y: self.x*rhs.sin() + self.y*rhs.cos(),
        }
    }
}

impl Mul<Translation2D> for Rotation2D {
    type Output = Translation2D;

    fn mul(self, rhs: Translation2D) -> Translation2D {
        Translation2D {
            x: rhs.x()*self.cos() - rhs.y()*self.sin(),
            y: rhs.x()*self.sin() + rhs.y()*self.cos(),
        }
    }
}

impl Neg for Translation2D {
    type Output = Translation2D;

    fn neg(self) -> Translation2D {
        Translation2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_add() {
        let translation1 = Translation2D::new(1.0, 2.0);
        let translation2 = Translation2D::new(4.0, 2.0);

        let translation3 = translation1 + translation2;
        assert!((translation3.x - 5.0) < 1e-10);
        assert!((translation3.y - 4.0) < 1e-10);
    }

    #[test]
    fn test_sub() {
        let translation1 = Translation2D::new(1.0, 2.0);
        let translation2 = Translation2D::new(4.0, 2.0);

        let translation3 = translation1 - translation2;
        assert!((translation3.x - -3.0) < 1e-10);
        assert!((translation3.y - 0.0) < 1e-10);
    }

    #[test]
    fn test_rot() {
        let translation1 = Translation2D::new(1.0, 2.0);
        let rot = Rotation2D::from_radians(PI / 2.0);

        let translation2 = translation1 * rot;
        assert!((translation2.x - -2.0) < 1e-10);
        assert!((translation2.y - 1.0) < 1e-10);
    }

    #[test]
    fn test_from_polar() {
        let translation = Translation2D::from_polar(2.0, Rotation2D::from_radians(2.0 * PI / 3.0));

        assert!((translation.x - 1.0) < 1e-10);
        assert!((translation.y - 3.0_f64.sqrt()) < 1e-10);
    }

    #[test]
    fn test_distance() {
        let translation1 = Translation2D::new(1.0, 4.0);

        let translation2 = Translation2D::new(3.0, 2.0);

        assert!((translation1.distance(translation2) - 8.0_f64.sqrt()).abs() < 1e-10);
    }
}