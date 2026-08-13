use crate::localization::odometry::odometry::Odometry;
use futures::stream::Forward;
use vexide::prelude::*;
use crate::localization::odometry::tracker::{
    dead_wheel_tracker::DealWheelTracker,
    drivetrain_tracker::DrivetrainTracker,
    tracker::Tracker,
    heading_source::HeadingSource,
};
use std::sync::{Arc, Mutex};

pub struct TwoTrackerOdom<F: Tracker, S: Tracker, H: HeadingSource> {
    forward_tracker: F,
    side_tracker: S,
    heading_source: H,
}

impl<F: Tracker, S: Tracker, H: HeadingSource> TwoTrackerOdom<F, S, H>  {
    pub fn new(forward: F, side: S, heading: H) -> Self {
        Self {forward_tracker: forward, side_tracker: side, heading_source: heading}
    }
}

impl<F: Tracker, S: Tracker, H: HeadingSource> Odometry for TwoTrackerOdom<F, S, H> {
    fn update(&mut self) {
        self.forward_tracker.update();
        self.side_tracker.update();
        self.heading_source.update();
    }

    fn forward_distance(&self) -> f64 {
        self.forward_tracker.get_distance_in()
    }

    fn sideways_distance(&self) -> f64 {
        self.side_tracker.get_distance_in()
    }

    /// Returns the heading in radians.
    fn heading(&self) -> f64 {
        self.heading_source.heading_radians()
    }

    fn calibrate_heading(&mut self) {
        self.heading_source.calibrate();
    }
}