use std::time::Duration;

/// Delay struct to handle hardware choice automatically.
pub struct Delay {}

impl Delay {
    /// waits the specified amount of time in milliseconds.
    pub async fn delay(millis: u64) {
        #[cfg(feature = "host")]
        {
            std::thread::sleep(Duration::from_millis(millis));
        }

        #[cfg(feature = "vex")]
        {
            use vexide::prelude::*;
            sleep(core::time::Duration::from_millis(millis)).await;
        }
    }
}