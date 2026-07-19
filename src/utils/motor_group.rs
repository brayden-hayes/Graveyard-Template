use vexide::prelude::*;

pub struct MotorGroup {
    motors: Vec<Motor>,
    reverse: bool,
}

impl MotorGroup {
    pub fn new(motors: Vec<Motor>, reverse: bool) -> Self {
        Self { motors, reverse }
    }

    pub fn spin(&mut self, voltage: f64) {
        let voltage = if self.reverse { -voltage } else { voltage };
        
        for motor in &mut self.motors {
            motor.set_voltage(voltage);
        }
    }
}