use crate::localization::odometry::odometry::Odometry;


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