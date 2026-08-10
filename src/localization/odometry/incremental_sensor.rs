use crate::utils::clock::Clock;


/// An artifical sensor that changes its reading by a specified increment every update.
/// Continues for a specified amount of time.
/// This can be used to chain multiple sensors together for more complex motion testing.
pub struct IncrementalSensor {
    value: f64,
    increment: f64,
    duration_ms: u64,
    clock: crate::utils::clock::GlobalClock,
    start_ms: u64,
}

impl IncrementalSensor {
    /// Creates a new IncrementalSensor with the specified starting value, increment amount, and duration in milliseconds.
    pub fn new(starting_value: f64, increment: f64, duration_millis: u64) -> Self {
        // Creates the clock to know when the duration is over.
        let clock = crate::utils::clock::create_clock();
        let start_ms = clock.now_ms();
        Self {
            value: starting_value,
            increment,
            duration_ms: duration_millis,
            clock,
            start_ms,
        }
    }

    /// Updates the Sensor reading by the specified increment every time this is called.
    pub fn update(&mut self) {
        // Check is the duration is over.
        let elapsed = self.clock.now_ms().saturating_sub(self.start_ms);
        if elapsed < self.duration_ms {
            //update value
            self.value += self.increment;
        }
    }

    // Gets the sensor value.
    pub fn get(&self) -> f64 {
        self.value
    }
}