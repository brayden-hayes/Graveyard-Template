use crate::geometry::{
    rotation2d::Rotation2D,
    translation2d::Translation2D,
    transform2d::Transform2D,
};

/// A finite local-frame motion in SE(2).
///
/// A Twist2D represents a local displacement (dx, dy) and a rotation
/// dtheta over a finite interval. It can be converted into a
/// Transform2D using the SE(2) exponential map.
///
/// Unlike a velocity twist, the components are finite increments rather
/// than rates of change.
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Twist2D {
    dx: f64,
    dy: f64,
    dtheta: f64,
}

impl Twist2D {

    /// Creates a new Twist2D with the zero values for dx, dy, and dtheta.
    pub fn zero() -> Self {
        Self {
            dx: 0.0,
            dy: 0.0,
            dtheta: 0.0,
        }
    }

    /// Creates a new Twist2D with the given dx, dy, and dtheta values.
    pub fn new(dx: f64, dy: f64, dtheta: f64) -> Self {
        Self {
            dx,
            dy,
            dtheta,
        }
    }

    /// Getter for the dx component of the twist.
    pub fn dx(&self) -> f64 {
        self.dx
    }

    /// Getter for the dy component of the twist.
    pub fn dy(&self) -> f64 {
        self.dy
    }

    /// Getter for the dtheta component of the twist.
    pub fn dtheta(&self) -> f64 {
        self.dtheta
    }

    /// Converts the twist into a Transform2D using the SE(2) exponential map.
    /// This is effectively the arc-based odometry motion model.
    pub fn exp(&self) -> Transform2D {
        let rotation = Rotation2D::from_radians(self.dtheta);

        // Prevent division by zero for small angles by assuming a straight-line path.
        if self.dtheta.abs() < 1e-9 {
            Transform2D::new(
                Translation2D::new(self.dx, self.dy),
                rotation,
            )
        } else {
            let sin_theta = self.dtheta.sin();
            let cos_theta = self.dtheta.cos();

            let s = sin_theta / self.dtheta;
            let c = (1.0 - cos_theta) / self.dtheta;

            Transform2D::new(
                Translation2D::new(
                    self.dx * s - self.dy * c,
                    self.dx * c + self.dy * s,
                ),
                rotation,
            )
        }
    }
}



/// Unit tests for the Twist2D struct and its methods.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twist_creation() {
        let twist = Twist2D::new(1.0, 2.0, 3.0);

        assert_eq!(twist.dx(), 1.0);
        assert_eq!(twist.dy(), 2.0);
        assert_eq!(twist.dtheta(), 3.0);
    }

    #[test]
    fn test_twist_exp_straight() {
        let twist = Twist2D::new(
            1.0,
            0.0,
            0.0
        );

        let transform = twist.exp();

        assert!((transform.translation().x() - 1.0).abs() < 1e-10);
        assert!(transform.translation().y().abs() < 1e-10);
    }

    #[test]
    fn test_twist_exp_curved() {
        let twist = Twist2D::new(
            1.0,
            0.0,
            std::f64::consts::PI / 2.0
        );

        let transform = twist.exp();

        let expected = 2.0 / std::f64::consts::PI;

        assert!(
            (transform.translation().x() - expected).abs() < 1e-10
        );

        assert!(
            (transform.translation().y() - expected).abs() < 1e-10
        );

        assert!(
            (transform.rotation().radians() - std::f64::consts::PI / 2.0).abs() < 1e-10
        );
    }
}