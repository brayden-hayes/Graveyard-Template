use super::transform2d::Transform2D;
use super::rotation2d::Rotation2D;
use super::translation2d::Translation2D;

#[derive(Clone, Copy)]
pub struct Twist2D {
    dx: f64,
    dy: f64,
    dtheta: f64,
}

impl Twist2D {
    pub fn new(dx: f64, dy: f64, dtheta: f64) -> Self {
        Self {
            dx,
            dy,
            dtheta,
        }
    }

    pub fn zero() -> Self {
        Self {
            dx: 0.0,
            dy: 0.0,
            dtheta: 0.0,
        }
    }

    pub fn dx(&self) -> f64 {
        self.dx
    }
    pub fn dy(&self) -> f64 {
        self.dy
    }
    pub fn dtheta(&self) -> f64 {
        self.dtheta
    }

    pub fn exp(&self) -> Transform2D {
        let rotation = Rotation2D::from_radians(self.dtheta);

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