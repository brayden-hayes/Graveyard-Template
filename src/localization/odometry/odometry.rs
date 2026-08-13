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
    fn calibrate_heading(&mut self);
}


pub struct OdometryPacket {
    x: f64,
    y: f64,
    theta: f64,
    t: u64,
}

impl OdometryPacket {
    pub fn new(x: f64, y: f64, theta: f64, t: u64) -> Self {
        Self {x, y, theta, t}
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn theta(&self) -> f64 {
        self.theta
    }

    pub fn t(&self) -> u64 {
        self.t
    }
}


/// Load odometry packets from CSV text.
///
/// Expected format (header optional):
/// ```text
/// x,y,theta
/// 0.0,0.0,0.0
/// 1.2,0.1,0.05
/// ```
///
/// - `x`     = absolute forward distance (inches)
/// - `y`     = absolute sideways distance (inches)
/// - `theta` = absolute heading (radians)
///
/// Blank lines and lines starting with `#` are ignored.
pub fn load_packets_from_csv(csv: &str) -> Vec<OdometryPacket> {
    csv.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with('#'))
        .filter(|line| {
            // skip header row like "x,y,theta"
            let lower = line.to_ascii_lowercase();
            !(lower.starts_with("x,") || lower == "x,y,theta")
        })
        .enumerate()
        .map(|(row, line)| {
            let cols: Vec<&str> = line.split(',').map(str::trim).collect();
            if cols.len() < 3 {
                panic!(
                    "CSV row {}: expected 3 columns (x,y,theta), got {} in {:?}",
                    row + 1,
                    cols.len(),
                    line
                );
            }

            let t: u64 = cols[0].parse().unwrap_or_else(|_| {
                panic!("CSV row {}: bad t value {:?}", row + 1, cols[3])
            });
            let x: f64 = cols[1].parse().unwrap_or_else(|_| {
                panic!("CSV row {}: bad x value {:?}", row + 1, cols[0])
            });
            let y: f64 = cols[2].parse().unwrap_or_else(|_| {
                panic!("CSV row {}: bad y value {:?}", row + 1, cols[1])
            });
            let theta: f64 = cols[3].parse().unwrap_or_else(|_| {
                panic!("CSV row {}: bad theta value {:?}", row + 1, cols[2])
            });
            

            // Match whatever constructor you have:
            OdometryPacket::new(x, y, theta, t)
            // If your packet is a plain struct:
            // OdometryPacket { x, y, theta }
        })
        .collect()
}