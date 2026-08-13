use vexide::prelude::*;
use std::f64::consts::PI;
pub trait Tracker: Send + Sync {
    fn get_distance_in(&self) -> f64;
    fn reset_position(&mut self);
    fn update(&mut self);
}