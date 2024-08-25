/// The interval at which to run the flight loop.
///
/// # Details
/// If interval value is negative, it is run in a certain number of frames based on the absolute value of the input.
/// If the interval is positive, it is a duration in seconds.
pub enum FlightLoopInterval {
    /// The interval is relative to the current time.
    RelativeToCurrent(f32),
    /// The interval is relative to the start of the sim.
    RelativeToStart(f32),
}
