use std::slice::SliceIndex;

use crate::localization::odometry::odometry::{Odometry, OdometryPacket};
pub struct LoggedOdometry {
    list: Vec<OdometryPacket>,
    i: usize,
}

impl LoggedOdometry {
    pub fn new(data: Vec<OdometryPacket>) -> Self {
        assert!(!data.is_empty(), "LoggedOdometry needs at least one packet");
        Self {list: data, i: 0}
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    fn current(&self) -> &OdometryPacket {
        // Clamp so we don't panic after the log ends
        let idx = self.i.min(self.list.len() - 1);
        &self.list[idx]
    }
}

impl Odometry for LoggedOdometry {
    fn update(&mut self) {
        if self.i + 1 < self.list.len() {
            self.i += 1;
        }
    }

    fn forward_distance(&self) -> f64 {
        self.list.get(self.i).unwrap().x()
    }

    fn sideways_distance(&self) -> f64 {
        self.list.get(self.i).unwrap().y()
    }

    fn heading(&self) -> f64 {
        self.list.get(self.i).unwrap().theta()
    }

    fn calibrate_heading(&mut self) {
        
    }
}