use crate::geometry::{
    rotation2d::Rotation2D,
    transform2d::Transform2D,
    translation2d::Translation2D
};
use std::ops::{Mul, MulAssign};

/// A 2D pose, consisting of a translation and a rotation.

/// It is the main representation of a global position and orientation.
/// It can be transformed by a Transform2D to get a new Pose2D, 
/// or can be used to get the relative Transform2D to another Pose2D.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pose2D {
    translation: Translation2D,
    rotation: Rotation2D,
}

impl Pose2D {
    /// Creates a new Pose2D with the zero values for translation and rotation.
    pub fn zero() -> Self {
        Self {
            translation: Translation2D::zero(),
            rotation: Rotation2D::zero(),
        }
    }

    /// Creates a new Pose2D with the given translation and rotation.
    pub fn new(translation: Translation2D, rotation: Rotation2D) -> Self {
        Self {
            translation,
            rotation,
        }
    }

    /// Getter for the x coordinate of the translation.
    pub fn x(&self) -> f64 {
        self.translation.x()
    }

    /// Getter for the y coordinate of the translation.
    pub fn y(&self) -> f64 {
        self.translation.y()
    }

    /// Getter for the translation of the pose.
    pub fn translation(&self) -> Translation2D {
        self.translation
    }

    /// Getter for the rotation of the pose.
    pub fn rotation(&self) -> Rotation2D {
        self.rotation
    }

    /// Transforms the pose by the given Transform2D, returning a new Pose2D.
    pub fn transform_by(&self, t: Transform2D) -> Pose2D {

        // Rotates the translation by the current rotation to get the global delta translation.
        let global_delta: Translation2D = self.rotation * t.translation();

        let new_translation = self.translation + global_delta;

        Pose2D {
            translation: new_translation,
            rotation: self.rotation * t.rotation(),
        }
    }

    /// Returns the Transform2D that transforms the other pose to this pose.
    /// Order matters, as this is not commutative. The returned Transform2D is the transform that, when applied to the other pose, will result in this pose.
    pub fn relative_to(&self, other: Pose2D) -> Transform2D {

        // Gets the translation difference between the two poses in world coordinates.
        let mut translation = self.translation - other.translation;

        // rotates the translation difference into the other pose's frame, converting it from world coordinates to the other pose's local coordinates.
        translation = other.rotation().inverse() * translation;

        // Gets the rotation difference between the two poses. rotation.inverse() * rotation is essentially the same as (-rotation) + rotation.
        let rotation = other.rotation().inverse() * self.rotation();

        Transform2D::new(translation, rotation)
    }

    /// Returns the distance between this pose and another pose, based on their translations.
    pub fn distance(&self, other: Pose2D) -> f64 {
        self.translation.distance(other.translation)
    }

    /// Returns the rotation from this pose to another pose, based on their translations.
    /// NOTE: This is different than the rotation difference between the two poses.
    /// This is the angle required to point from this pose's translation to the other pose's translation, in world coordinates.
    pub fn rotation_to(&self, other: Pose2D) -> Rotation2D {
        let delta = other.translation - self.translation;
        Rotation2D::from_radians(
            delta.y().atan2(delta.x())
        )
    }

    /// Returns the inverse of the transform.
    /// This is the transform that, when applied to this pose, will result in the identity pose (zero translation and zero rotation).
    pub fn inverse(&self) -> Transform2D {
        let inverse_rot = self.rotation.inverse();
        let inverse_translation: Translation2D = inverse_rot*(-self.translation);

        Transform2D::new(
            inverse_translation,
            inverse_rot,
        )
    }
}

/// Implementations of the multiplication operator for Pose2D and Transform2D.
/// This allows for the use of the * operator to act as a shorthand for the transform_by method.
impl Mul<Transform2D> for Pose2D {
    type Output = Pose2D;

    fn mul(self, rhs: Transform2D) -> Pose2D {
        self.transform_by(rhs)
    }
}

/// Implementations of the multiplication assignment operator for Pose2D and Transform2D.
/// This allows for the use of the *= operator to act as a shorthand for the transform_by method, modifying the original Pose2D in place.
impl MulAssign<Transform2D> for Pose2D {
    fn mul_assign(&mut self, rhs: Transform2D) {
        *self = self.transform_by(rhs);
    }
}



/// Unit tests for the Pose2D struct and its methods.
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