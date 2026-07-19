use super::rotation2d::Rotation2D;
use super::translation2d::Translation2D;

#[derive(Clone, Copy)]
pub struct Transform2D {
    translation: Translation2D,
    rotation: Rotation2D,
}


impl Transform2D {
    pub fn new(translation: Translation2D, rotation: Rotation2D) -> Self {
        Self {
            translation,
            rotation,
        }
    }
    pub fn zero() -> Self {
        Self {
            translation: Translation2D::zero(),
            rotation: Rotation2D::zero(),
        }
    }
    pub fn translation(&self) -> Translation2D {
        self.translation
    }

    pub fn rotation(&self) -> Rotation2D {
        self.rotation
    }

    pub fn x(&self) -> f64 {
        self.translation.x()
    }
    pub fn y(&self) -> f64 {
        self.translation.y()
    }
}