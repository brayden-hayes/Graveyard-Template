use super::rotation2d::Rotation2D;
use super::translation2d::Translation2D;
use super::transform2d::Transform2D;
use std::ops::Mul;


#[derive(Clone, Copy)]
pub struct Pose2D {
    translation: Translation2D,
    rotation: Rotation2D,
}

impl Pose2D {
    pub fn zero() -> Self {
        Self {
            translation: Translation2D::zero(),
            rotation: Rotation2D::zero(),
        }
    }

    pub fn new(translation: Translation2D, rotation: Rotation2D) -> Self {
        Self {
            translation,
            rotation,
        }
    }
    pub fn x(&self) -> f64 {
        self.translation.x()
    }

    pub fn y(&self) -> f64 {
        self.translation.y()
    }

    pub fn rotation(&self) -> Rotation2D {
        self.rotation
    }

    pub fn translation(&self) -> Translation2D {
        self.translation
    }

    pub fn transform_by(&self, t: Transform2D) -> Pose2D {
        let global_delta: Translation2D = self.rotation * t.translation();

        let new_translation = self.translation + global_delta;

        Pose2D {
            translation: new_translation,
            rotation: self.rotation * t.rotation(),
        }
    }

    pub fn relative_to(&self, other: Pose2D) -> Transform2D {
        let mut translation = self.translation - other.translation;
        translation = other.rotation().inverse() * translation;
        let rotation = other.rotation().inverse() * self.rotation();

        Transform2D::new(translation, rotation)
    }

    pub fn distance(&self, other: Pose2D) -> f64 {
        self.translation.distance(other.translation)
    }

    pub fn rotation_to(&self, other: Pose2D) -> Rotation2D {
        let delta = other.translation - self.translation;
        Rotation2D::from_radians(
            delta.y().atan2(delta.x())
        )
    }

    pub fn inverse(&self) -> Transform2D {
        let inverse_rot = self.rotation.inverse();
        let inverse_translation: Translation2D = inverse_rot*(-self.translation);

        Transform2D::new(
            inverse_translation,
            inverse_rot,
        )
    }
}

impl Mul<Transform2D> for Pose2D {
    type Output = Pose2D;

    fn mul(self, rhs: Transform2D) -> Pose2D {
        self.transform_by(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_get_x_y_rot() {
        let translation = Translation2D::new(1.0, 2.0);
        let rotation = Rotation2D::from_radians(PI / 2.0);
        let pose = Pose2D::new(translation, rotation);

        assert_eq!(pose.x(), translation.x());
        assert_eq!(pose.y(), translation.y());
        assert_eq!(pose.rotation().radians(), rotation.radians());
    }

    #[test]
    fn test_tranform_x() {
        let pose = Pose2D::new(
            Translation2D::new(0.0, 0.0),
            Rotation2D::from_radians(0.0)
        );

        let transformation = Transform2D::new(
            Translation2D::new(1.0, 0.0),
            Rotation2D::from_radians(0.0)
        );

        let transformed_pose = pose.transform_by(transformation);

        assert_eq!(transformed_pose.x(), 1.0);
    }

    #[test]
    fn test_transform_rotated() {
        let pose = Pose2D::new(
            Translation2D::new(0.0, 0.0),
            Rotation2D::from_radians(PI / 2.0)
        );

        let transformation = Transform2D::new(
            Translation2D::new(1.0, 0.0),
            Rotation2D::from_radians(0.0)
        );

        let transformed_pose = pose.transform_by(transformation);

        assert!((transformed_pose.x() - 0.0).abs() < 1e-10);
        assert!((transformed_pose.y() - 1.0).abs() < 1e-10);
        assert_eq!(transformed_pose.rotation().radians(), PI / 2.0);
    }

    #[test]
    fn test_relative_to_no_rot() {
        let pose1 = Pose2D::new(
            Translation2D::new(0.0, 0.0),
            Rotation2D::from_radians(PI / 2.0),
        );
        let pose2 = Pose2D::new(
            Translation2D::new(0.0, 1.0),
            Rotation2D::from_radians(PI / 2.0),
        );

        let transform = pose2.relative_to(pose1);
        assert!((transform.x() - 1.0) < 1e-10);
        assert!((transform.y() - 0.0) < 1e-10);
        assert!((transform.rotation().radians() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_relative_to_with_rot() {
        let pose1 = Pose2D::new(
            Translation2D::new(0.0, 0.0),
            Rotation2D::from_radians(PI / 2.0),
        );
        let pose2 = Pose2D::new(
            Translation2D::new(0.0, 1.0),
            Rotation2D::from_radians(PI),
        );

        let transform = pose2.relative_to(pose1);
        assert!((transform.x() - 1.0) < 1e-10);
        assert!((transform.y() - 0.0) < 1e-10);
        assert!((transform.rotation().radians() - PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_rotation_to() {
        let pose1 = Pose2D::new(
            Translation2D::new(1.0, 4.0),
            Rotation2D::zero(),
        );

        let pose2 = Pose2D::new(
            Translation2D::new(3.0, 2.0),
            Rotation2D::zero(),
        );
        assert!((pose1.rotation_to(pose2).radians() - -0.785).abs() < 1e-3);
    }

    #[test]
    fn test_inverse() {
        let pose = Pose2D::new(
            Translation2D::new(1.0, 4.0),
            Rotation2D::from_radians(PI / 3.0),
        );
        let inverse = pose.inverse();
        let combined = pose*inverse;

        assert!(combined.x().abs() < 1e-10);
        assert!(combined.y().abs() < 1e-10);
        assert!(combined.rotation().radians().abs() < 1e-10);
    }

    #[test]
    fn test_translation() {
        let pose = Pose2D::new(
            Translation2D::new(3.0, 1.0),
            Rotation2D::zero(),
        );
        let translation = pose.translation();
        assert_eq!(translation.x(), 3.0);
        assert_eq!(translation.y(), 1.0);
    }
}