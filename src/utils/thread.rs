use std::future;

/// Thread struct to handle specific hardware choice automatically.
/// Designed to run both on PC and the VEX V5 brain with no code changes required.
pub struct Thread {}

impl Thread {
    /// Fire-and-forget background work (sync), std::thread-like.
    pub fn spawn<F>(f: F) 
    where 
        F: FnOnce() + Send + 'static,
    {
        #[cfg(feature = "host")]
        {
            let _ = std::thread::spawn(f);
        }

        #[cfg(feature = "vex")]
        {
            use vexide::prelude::*;
            use vexide::task;

            task::spawn(async move {
                f();
            })
            .detach();
        }
    }

    /// Fire-and-forget background async task.
    pub fn spawn_async<F, T>(future: F)
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        #[cfg(feature = "host")]
        {
            // Run the future on its own OS thread.
            let _ = std::thread::spawn(move || {
                futures::executor::block_on(future);
            });
        }

        #[cfg(feature = "vex")]
        {
            use vexide::prelude::*;
            use vexide::task;
            task::spawn(future).detach();
        }
    }
}