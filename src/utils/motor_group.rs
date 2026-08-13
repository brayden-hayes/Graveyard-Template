use std::vec;

use vexide::prelude::*;

/// Custom implementation of a MotorGroup to allow joint control of several motors.
pub struct MotorGroup {
    motors: Vec<Motor>,
    reverse: bool,
}

impl MotorGroup {
    /// Creates the motor group. Requires a vector of motors and a reversed boolean.
    pub fn new(motors: Vec<Motor>, reverse: bool) -> Self {
        Self { motors, reverse }
    }

    /// Sets the voltage of all motors in the group.
    pub fn spin(&mut self, voltage: f64) {
        let voltage = if self.reverse { -voltage } else { voltage };
        
        for motor in &mut self.motors {
            motor.set_voltage(voltage);
        }
    }

    pub fn get_position_radians(&self) -> f64 {
        let total_rotation: f64 = self.motors.iter()
            .map(|motor| motor.position().unwrap().as_radians())
            .sum();
        total_rotation / self.motors.len() as f64
    }

    pub fn reset_position(&mut self) {
        self.motors.iter_mut().map(|motor| motor.reset_position());
    }
}