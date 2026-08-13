use vexide::{math::Angle, prelude::*, smart::imu};
use crate::utils::delay::Delay;
pub trait HeadingSource: Send + Sync {
    /// Returns the current heading in radians.
    fn heading_radians(&self) -> f64;

    fn set_angle_radians(&mut self, radians: f64);

    fn update(&mut self);

    fn calibrate(&mut self);
}

pub struct IMU {
    inertial: InertialSensor,
}

impl HeadingSource for IMU {
    fn heading_radians(&self) -> f64 {
        self.inertial.heading().unwrap().as_radians()
    }

    fn set_angle_radians(&mut self, radians: f64) {
        self.inertial.set_heading(Angle::from_radians(radians));
    }

    fn update(&mut self) {
        
    }

    fn calibrate(&mut self) {
        self.inertial.calibrate();

        while self.inertial.is_calibrating().unwrap() {
            Delay::delay(10);
        }
    }
}

impl IMU {
    pub fn new(inertial: InertialSensor) -> Self {
        Self {inertial}
    }
}