use vexide::prelude::*;
use crate::localization::odometry::tracker::tracker::Tracker;
use std::f64::consts::PI;
pub struct DealWheelTracker {
    rotation_sensor: RotationSensor,
    wheel_diameter: f64,
    sensor_scale: f64,
}

impl DealWheelTracker {
    pub fn new(rotation_sensor: RotationSensor, wheel_diameter: f64, sensor_scale: f64) -> Self {
        Self {rotation_sensor, wheel_diameter, sensor_scale}
    }

    pub fn get_rotation_radians(&self) -> f64 {
        self.rotation_sensor.angle().unwrap().as_radians()
    }

    pub fn get_rotation_degrees(&self) -> f64 {
        self.rotation_sensor.angle().unwrap().as_degrees()
    }
}

impl Tracker for DealWheelTracker {
    fn get_distance_in(&self) -> f64 {
        let angle = self.get_rotation_radians();
        let rotations = angle / (2.0 * PI);
        let distance = rotations * PI * self.wheel_diameter;
        distance * self.sensor_scale
    }
    
    fn reset_position(&mut self) {
        self.rotation_sensor.reset_position();
    }

    fn update(&mut self) {
        
    }
}