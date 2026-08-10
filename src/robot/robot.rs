use crate::geometry::translation2d::Translation2D;
use crate::geometry::{pose2d::Pose2D, twist2d::Twist2D, transform2d::Transform2D, rotation2d::Rotation2D};
use crate::localization::odometry::odometry::Odometry;
use vexide::prelude::*;
use std::sync::{Mutex, Arc};
use vexide::task;
use std::time::Duration;


/// Collection of previous odometry reading values.
pub struct PreviousMeasurement {
    pub x: f64,
    pub y: f64,
    pub rot: Rotation2D,
}

/// The current state of the robot including pose and the previous odometry readings.
pub struct RobotState {
    pose: Mutex<Pose2D>,
    prev_measurement: Mutex<PreviousMeasurement>,
}

impl RobotState {
    /// Updates the robot state odometry and previous readings.
    fn update<O: Odometry>(&self, odometry: &mut O) {
        
        // Gets absolute sensor readings from Odometry object.
        let x = odometry.forward_distance();
        let y = odometry.sideways_distance();
        let rotation = Rotation2D::from_radians(odometry.heading());

        // Lock the prev_measurement mutex.
        let mut previous = self.prev_measurement.lock().unwrap();

        // Computes the local deltas since last loop.
        let dx = x - previous.x;
        let dy = y - previous.y;
        let d_rot = rotation * previous.rot.inverse();

        // Updates the previous values
        previous.x = x;
        previous.y = y;
        previous.rot = rotation;

        // Converts the body-frame motion increment into a rigid-body transform
        // using the SE(2) exponential map.
        let delta_pose = Twist2D::new(dx, dy, d_rot.radians()).exp();

        // Attempts to lock the pose mutex
        let mut pose = self.pose.lock().unwrap();

        // Transforms the pose by the delta_pose transform.
        *pose = pose.transform_by(delta_pose);
    }
}


/// The Robot struct controls autonomous/teleop methods as well as custom localization methods.
/// Contains a RobotState and generic odometry type to allow easy replacement of measurement readings.
/// This also allows for testing with man-made sensor data without a physical robot.
pub struct Robot<O: Odometry> {
    odometry: Arc<Mutex<O>>,
    state: Arc<RobotState>,
}

impl<O: Odometry + 'static> Robot<O> {

    /// Creates a new robot with Pose2D (0, 0, 0), and provided Odometry.
    pub fn new(odometry: O) -> Self {
        Self {
            odometry: Arc::new(Mutex::new(odometry)),
            state: Arc::new(RobotState {
                pose: (Mutex::new(Pose2D::zero())),
                prev_measurement: (Mutex::new(PreviousMeasurement {
                    x: 0.0,
                    y: 0.0,
                    rot: Rotation2D::zero(),
                })),
            }),
        }
    }

    /// Sets the robot's pose to the given Pose2D.
    pub fn set_pose(&self, pose: Pose2D) {
        *self.state.pose.lock().unwrap() = pose;
    }

    /// Getter for the robot's pose.
    pub fn pose(&self) -> Pose2D {
        *self.state.pose.lock().unwrap()
    }

    /// Starts the localization thread.
    /// Contains two different methods depending on if it was run on PC or the VEX Brain.
    pub fn start_localization(&self) {

        // Clones the state and odometry to allow editing.
        let state = Arc::clone(&self.state);
        let odometry = Arc::clone(&self.odometry);

        // Spawns a Thread that performs the localization update.
        // The specific thread type is handled in `thread.rs`.
        crate::utils::thread::Thread::spawn_async(async move {
            loop {
                {
                    // Locks the odometry mutex to allow editing.
                    let mut odom = odometry.lock().unwrap();
                    // Updates the odometry is applicable. Used mainly for fake sensor data.
                    odom.update();
                    // Updates the state with the new odometry data.
                    state.update(&mut *odom);
                }
                // 10 ms delay to prevent using all the Brain's resources.
                crate::utils::delay::Delay::delay(10).await;
            }
        });
    }

}