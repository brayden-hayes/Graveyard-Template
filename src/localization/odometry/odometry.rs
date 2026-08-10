use std::f64::consts::PI;
use nalgebra::Const;

use crate::{geometry::rotation2d::Rotation2D, utils::clock::Clock};

/// Odometry trait that requires the implementation of forward_distance, sideways_distance, and heading.
/// These functions are used by the Robot struct to retrieve sensor data for position tracking.
pub trait Odometry: Send + Sync {
    fn update(&mut self);
    fn forward_distance(&self) -> f64;
    fn sideways_distance(&self) -> f64;
    fn heading(&self) -> f64;
}