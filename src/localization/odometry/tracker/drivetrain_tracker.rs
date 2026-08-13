use vexide::prelude::*;
use crate::localization::odometry::{odometry::Odometry, tracker::tracker::Tracker};
use crate::utils::motor_group::MotorGroup;
use std::sync::{Arc, Mutex};
use std::f64::consts::PI;


pub trait DriveSide: Send + Sync {
    fn position_radians(&self) -> f64;
    fn reset_position(&self);
}

pub struct DrivetrainTracker {
    left_drive: Arc<Mutex<dyn DriveSide>>,
    right_drive: Arc<Mutex<dyn DriveSide>>,
    gear_ratio: f64, // Output / Input
    wheel_size: f64,
    sensor_scale: f64,
}

impl DrivetrainTracker {
    pub fn new(
        left_drive: Arc<Mutex<dyn DriveSide>>,
        right_drive: Arc<Mutex<dyn DriveSide>>,
        gear_ratio: f64, // Output / Input
        wheel_size: f64,
        sensor_scale: f64,
    ) -> Self {
        Self{left_drive, right_drive, gear_ratio, wheel_size, sensor_scale}
    }

    pub fn get_average_rotation_radians(&self) -> f64 {
        let left_rot = self.left_drive.lock().unwrap().position_radians();
        let right_rot = self.right_drive.lock().unwrap().position_radians();

        (left_rot + right_rot) / 2.0
    }
}

impl Tracker for DrivetrainTracker {
    fn get_distance_in(&self) -> f64 {
        let angle = self.get_average_rotation_radians();
        let rotations = angle / (2.0 * PI);
        let distance = rotations * PI * self.wheel_size * self.gear_ratio;
        distance * self.sensor_scale
    }

    fn reset_position(&mut self) {
        self.left_drive.lock().unwrap().reset_position();
        self.right_drive.lock().unwrap().reset_position();
    }

    fn update(&mut self) {
        
    }
}