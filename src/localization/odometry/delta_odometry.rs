use crate::localization::odometry::{odometry::Odometry, incremental_sensor::IncrementalSensor};

/// An artificial odometry that implements IncrementalSensors to change position real-time.
pub struct DeltaOdometry {
    forward: IncrementalSensor,
    side: IncrementalSensor,
    rot: IncrementalSensor,
}


impl DeltaOdometry {

    /// Creates a new DeltaOdometry with the IncrementalSensors set to 0.
    pub fn new() -> Self {
        Self {  
            forward: IncrementalSensor::new(0.0, 0.0, 0),
            side: IncrementalSensor::new(0.0, 0.0, 0),
            rot: IncrementalSensor::new(0.0, 0.0, 0),
        }
    }
    
    /// Set the forward IncrementalSensor parameters.
    pub fn set_forward_params(&mut self, start_x: f64, inc_x: f64, dur_x: u64) {
        self.forward = IncrementalSensor::new(start_x, inc_x, dur_x);
    }

    /// Set the side IncrementalSensor parameters.
    pub fn set_side_params(&mut self, start_y: f64, inc_y: f64, dur_y: u64) {
        self.side = IncrementalSensor::new(start_y, inc_y, dur_y);
    }

    /// Set the rotational IncrementalSensor parameters.
    pub fn set_rot_params(&mut self, start_h: f64, inc_h: f64, dur_h: u64) {
        self.rot = IncrementalSensor::new(start_h, inc_h, dur_h);
    }
}

impl Odometry for DeltaOdometry {

    /// Updates the IncrementalSensors each time this is called.
    fn update(&mut self) {
        self.forward.update();
        self.side.update();
        self.rot.update();
    }

    /// Gets the forward sensor reading.
    fn forward_distance(&self) -> f64 {
        self.forward.get()
    }

    /// Gets the side sensor reading.
    fn sideways_distance(&self) -> f64 {
        self.side.get()
    }

    /// Gets the heading reading.
    fn heading(&self) -> f64 {
        self.rot.get()
    }
}