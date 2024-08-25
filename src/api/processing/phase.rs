/// The phase of the flight loop.
pub enum FlightLoopPhaseType {
    /// The callback runs before X-Plane integrates the flight model.
    BeforeFlightModel,
    /// The callback runs after X-Plane integrates the flight model.
    AfterFlightModel,
}

impl FlightLoopPhaseType {
    /// Converts the flight loop phase type to the X-Plane nativ type
    pub fn native(self) -> i32 {
        match self {
            FlightLoopPhaseType::BeforeFlightModel => 0,
            FlightLoopPhaseType::AfterFlightModel => 1,
        }
    }
}
