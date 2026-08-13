use vexide::{prelude::*};
mod utils;
mod geometry;
mod robot;
mod localization;
use crate::robot::robot::Robot;
use crate::localization::odometry::{
    constant_odometry::ConstantOdometry,
    odometry::Odometry,
    delta_odometry::DeltaOdometry,
    two_tracker_odom::TwoTrackerOdom,
};
use crate::localization::odometry::tracker::{dead_wheel_tracker::DealWheelTracker, heading_source::IMU};

#[cfg(feature = "vex")]
#[vexide::main]
async fn main(peripherals: Peripherals) {

    // Example of how to create the odometry object on the real robot.

    let fake_odom = ConstantOdometry::new(0.0, 0.0, 0.0);
    let forward_sensor = RotationSensor::new(peripherals.port_1, Direction::Forward);
    let side_sensor = RotationSensor::new(peripherals.port_10, Direction::Forward);
    let forward_tracker = DealWheelTracker::new(forward_sensor, 2.0, 1.0);
    let side_tracker = DealWheelTracker::new(side_sensor, 2.0, 1.0);
    let inertial = IMU::new(InertialSensor::new(peripherals.port_11));
    let two_tracker_odom = TwoTrackerOdom::new(forward_tracker, side_tracker, inertial);

    let mut robot = Robot::new(two_tracker_odom);
    robot.calibrate_heading_source();

    // Required for competition. Waits for the autonomous and driver callbacks in `competition.rs`.
    robot.compete().await;
}


/// THIS FUNCTION IS NEVER CALLED
/// This is an example of how to test the odometry system on a PC instead of the robot.
/// To run this on PC, open the command line in VSCode and run `cargo test test_localization --features host --no-default-features -- --nocapture`.
/// You can then read the printed coordinates or open the created svg to visualize the path.

#[cfg(feature = "host")]
#[cfg(test)]
mod test_localization_delta_odom {
    use crate::geometry::pose2d::Pose2D;
    use crate::robot::robot::Robot;
    use crate::localization::odometry::{delta_odometry::DeltaOdometry, constant_odometry::ConstantOdometry, odometry::OdometryPacket};
    use super::*;
    #[test]
    fn test_localization_delta_odom() {

        // Create the DeltaOdometry and set the motion parameters.
        let mut fake_odom = DeltaOdometry::new();
        fake_odom.set_forward_params(50.0, 0.5, 1000);  // Starts at 50". Reads 0.5" every cycle for 1000 ms.
        fake_odom.set_side_params(50.0, 0.01, 100);     // Starts at 50". Reads 0.01" every cycle for 100 ms.
        fake_odom.set_rot_params(0.0, 0.04, 900);        // Starts at 0.0 deg. Reads 0.1 deg every cycle for 200 ms.

        // Crate the robot and begin position tracking.
        let mut robot = Robot::new(fake_odom);
        robot.calibrate_heading_source();
        robot.start_localization();

        // Short delay to prevent a position jump in the svg.
        // Purely for visualization, no real impact.
        std::thread::sleep(std::time::Duration::from_millis(10));

        

        let mut poses: Vec<Pose2D> = Vec::new();

        // Repeats 100 times with a 10 ms delay. 100 * 10 ms = 1000 ms.
        // Set this to the longest time needed depending on the motion parameters.

        for i in 0..100 {

            // Gets the pose and pushes to the list.
            let p = robot.pose();
            poses.push(p);

            // Prints the coordinates to the terminal.
            println!("{:.3}, {:.3}, {:.3}, {:.3}", p.x(), p.y(), p.rotation().radians(), i);
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        // Creates the svg file.
        crate::utils::visualizer::write_trajectory_svg(&poses, "trajectory.svg").unwrap();
        println!("Wrote trajectory.svg – open it in a browser");
    }
}




#[cfg(feature = "host")]
#[cfg(test)]
mod test_localization_logged_odom {
    use crate::geometry::{pose2d::Pose2D, translation2d::Translation2D, rotation2d::Rotation2D};
    use crate::robot::robot::Robot;
    use crate::localization::odometry::{delta_odometry::DeltaOdometry, constant_odometry::ConstantOdometry, odometry::OdometryPacket, logged_odometry::LoggedOdometry, odometry::load_packets_from_csv};
    use super::*;
    #[test]
    fn test_localization_logged_odom() {

        let packets = load_packets_from_csv(include_str!("path1.csv"));
        let odom = LoggedOdometry::new(packets);
        let mut robot = Robot::new(odom);
        robot.set_pose(Pose2D::new(
            Translation2D::new(40.0, 40.0),
            Rotation2D::from_radians(0.896055),
        ));
        robot.calibrate_heading_source();

        robot.start_localization();

        // Short delay to prevent a position jump in the svg.
        // Purely for visualization, no real impact.
        std::thread::sleep(std::time::Duration::from_millis(10));

        

        let mut poses: Vec<Pose2D> = Vec::new();

        // Repeats 100 times with a 10 ms delay. 100 * 10 ms = 1000 ms.
        // Set this to the longest time needed depending on the motion parameters.

        for i in 0..100 {

            // Gets the pose and pushes to the list.
            let p = robot.pose();
            poses.push(p);

            // Prints the coordinates to the terminal.
            println!("{:.3}, {:.3}, {:.3}, {:.3}", p.x(), p.y(), p.rotation().radians(), i);
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        // Creates the svg file.
        crate::utils::visualizer::write_trajectory_svg(&poses, "trajectory.svg").unwrap();
        println!("Wrote trajectory.svg – open it in a browser");
    }
}
