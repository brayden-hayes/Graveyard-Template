use vexide::prelude::*;
use crate::robot::odometry::Odometry;
use crate::robot::robot::Robot;

/// Competition callbacks for autonomous and driver control.
impl<O: Odometry + 'static> Compete for Robot<O> {
    async fn autonomous(&mut self) {
        println!("Autonomous!");

        // Start the position tracking thread.
        self.start_localization();
    }

    async fn driver(&mut self) {
        println!("Driver!");
    }
}