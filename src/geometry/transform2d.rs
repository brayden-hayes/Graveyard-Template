use crate::geometry::{
    rotation2d::Rotation2D,
    translation2d::Translation2D
};

/// A 2D transformation, consisting of a translation and a rotation.
/// It is the main representation of a transformation in 2D space, and can be used to transform poses and twists.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform2D {
    translation: Translation2D,
    rotation: Rotation2D,
}


impl Transform2D {

    /// Creates a new Transform2D with the zero values for translation and rotation.
     pub fn zero() -> Self {
        Self {
            translation: Translation2D::zero(),
            rotation: Rotation2D::zero(),
        }
    }

    /// Creates a new Transform2D with the given translation and rotation.
    pub fn new(translation: Translation2D, rotation: Rotation2D) -> Self {
        Self {
            translation,
            rotation,
        }
    }

    /// Getter for the translation of the transform.
    pub fn translation(&self) -> Translation2D {
        self.translation
    }

    /// Getter for the rotation of the transform.
    pub fn rotation(&self) -> Rotation2D {
        self.rotation
    }

    /// Getter for the x coordinate of the translation.
    pub fn x(&self) -> f64 {
        self.translation.x()
    }

    /// Getter for the y coordinate of the translation.
    pub fn y(&self) -> f64 {
        self.translation.y()
    }
}



/// Unit tests for the Transform2D struct and its methods.
#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    use crate::geometry::{
        rotation2d::Rotation2D,
        translation2d::Translation2D,
    };

    #[test]
    fn zero_constructor_returns_zero_transform() {
        let t = Transform2D::zero();

        assert_eq!(t.translation(), Translation2D::zero());
        assert_eq!(t.rotation(), Rotation2D::zero());
        assert_eq!(t.x(), 0.0);
        assert_eq!(t.y(), 0.0);
    }

    #[test]
    fn new_constructor_stores_values() {
        let translation = Translation2D::new(3.5, -2.25);
        let rotation = Rotation2D::from_radians(1.234);

        let t = Transform2D::new(translation, rotation);

        assert_eq!(t.translation(), translation);
        assert_eq!(t.rotation(), rotation);
        assert_eq!(t.x(), 3.5);
        assert_eq!(t.y(), -2.25);
    }

    #[test]
    fn getters_return_expected_values() {
        let t = Transform2D::new(
            Translation2D::new(-10.0, 42.5),
            Rotation2D::from_radians(PI / 2.0),
        );

        assert_eq!(t.translation(), Translation2D::new(-10.0, 42.5));
        assert_eq!(t.rotation(), Rotation2D::from_radians(PI / 2.0));
        assert_eq!(t.x(), -10.0);
        assert_eq!(t.y(), 42.5);
    }
}