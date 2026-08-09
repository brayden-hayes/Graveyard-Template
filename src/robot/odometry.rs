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

/// The ConstantOdometry struct is used to pass in constant data, used for simple tests.
pub struct ConstantOdometry {
    forward_value: f64,
    side_value: f64,
    rot_value: f64,
}

impl ConstantOdometry {
    /// Creates an empty ConstantOdometry object with the specified values.
    pub fn new(forward_value: f64, side_value: f64, rot_value: f64) -> Self {
        Self{forward_value, side_value, rot_value}
    }
}

impl Odometry for ConstantOdometry {
    /// Required update function. Since ConstantOdometry doesn't need updating, it is left empty. 
    fn update(&mut self) {
        
    }
    /// Gets the forward reading.
    /// THIS IS NOT A DELTA. The sensor will always read this value and the robot will not move.
    fn forward_distance(&self) -> f64 {
        self.forward_value
    }
    /// Gets the side reading.
    /// THIS IS NOT A DELTA. The sensor will always read this value and the robot will not move.
    fn sideways_distance(&self) -> f64 {
        self.side_value
    }
    /// Gets the heading.
    /// THIS IS NOT A DELTA. The sensor will always read this value and the robot will not move.
    fn heading(&self) -> f64 {
        self.rot_value
    }
}

/// An artifical sensor that changes its reading by a specified increment every update.
/// Continues for a specified amount of time.
/// This can be used to chain multiple sensors together for more complex motion testing.
pub struct IncrementalSensor {
    value: f64,
    increment: f64,
    duration_ms: u64,
    clock: crate::utils::clock::GlobalClock,
    start_ms: u64,
}

impl IncrementalSensor {
    /// Creates a new IncrementalSensor with the specified starting value, increment amount, and duration in milliseconds.
    pub fn new(starting_value: f64, increment: f64, duration_millis: u64) -> Self {
        // Creates the clock to know when the duration is over.
        let clock = crate::utils::clock::create_clock();
        let start_ms = clock.now_ms();
        Self {
            value: starting_value,
            increment,
            duration_ms: duration_millis,
            clock,
            start_ms,
        }
    }

    /// Updates the Sensor reading by the specified increment every time this is called.
    pub fn update(&mut self) {
        // Check is the duration is over.
        let elapsed = self.clock.now_ms().saturating_sub(self.start_ms);
        if elapsed < self.duration_ms {
            //update value
            self.value += self.increment;
        }
    }

    // Gets the sensor value.
    pub fn get(&self) -> f64 {
        self.value
    }
}

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