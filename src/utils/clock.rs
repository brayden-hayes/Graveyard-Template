/// Clock trait to allow one clock for both Hardware and PC.
pub trait Clock: Send + Sync {
    /// Get current time in micro seconds.
    fn now_us(&self) -> u64;

    /// Get current time in milliseconds.
    fn now_ms(&self) -> u64 {
        self.now_us() / 1000
    }

    /// Get current time in seconds.
    fn now_s(&self) -> f64 {
        self.now_us() as f64 / 1_000_000.0
    }
}

/// Specific to PC utilization. Uses the std::time instead of vexide.
#[cfg(feature = "host")]
pub struct HostClock {
    start: std::time::Instant,
}

#[cfg(feature = "host")]
impl HostClock {
    pub fn new() -> Self {
        Self {
            start: std::time::Instant::now(),
        }
    }
}

#[cfg(feature = "host")]
impl Clock for HostClock {
    fn now_us(&self) -> u64 {
        self.start.elapsed().as_micros() as u64
    }
}


/// Specific to VEX Hardware utilization.
/// Runs as the default unless you run --features host --no-default-features in the command line.
#[cfg(feature = "vex")]
pub struct VexClock;

#[cfg(feature = "vex")]
impl VexClock {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(feature = "vex")]
impl Clock for VexClock {
    fn now_us(&self) -> u64 {
        // Vexide provides a monotonic micros counter
        (vexide::time::user_uptime().as_micros()) as u64
    }
}


// ---------- Convenient alias ----------
// These allow the creation of one GlobalClock object
// instead of using both VexClock and HostClock in different circumstances.
#[cfg(feature = "host")]
pub type GlobalClock = HostClock;

#[cfg(feature = "vex")]
pub type GlobalClock = VexClock;

/// returns a new GlobalClock that gets the current time since creation.
pub fn create_clock() -> GlobalClock {
    GlobalClock::new()
}