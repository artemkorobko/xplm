use super::{destroy_flight_loop, ProcessingError};

pub struct FlightLoopId(xplm_sys::XPLMFlightLoopID);

impl FlightLoopId {
    /// Returns X-Plane native flight loop identifier.
    pub fn native(&self) -> xplm_sys::XPLMFlightLoopID {
        self.0
    }
}

impl TryFrom<xplm_sys::XPLMFlightLoopID> for FlightLoopId {
    type Error = ProcessingError;

    fn try_from(value: xplm_sys::XPLMMenuID) -> std::result::Result<Self, ProcessingError> {
        if value.is_null() {
            Err(Self::Error::InvalidFlightLoopId)
        } else {
            Ok(Self(value))
        }
    }
}

/// A flight loop callback handler.
pub trait FlightLoopHandler: 'static {
    /// Each time the flight loop is iterated through, you receive this call at the end.
    /// Flight loop callbacks receive a number of input timing parameters. These input timing parameters are not
    /// particularly useful. You may need to track your own timing data (e.g. by reading datarefs).
    ///
    /// # Arguments
    /// * `elapsed_since_last_call` - elapsed time since last call in seconds.
    /// * `elapsed_since_last_flight_loop` - elapsed time since last flight loop call in seconds.
    /// * `counter` - a monotonically increasing counter, bumped once per flight loop dispatch from the sim.
    ///
    /// # Returns
    /// Returns a value which controls when the callback will next be called.
    /// Return 0 to stop receiving callbacks.
    /// Return a positive number to specify how many seconds until the next callback.
    /// Return a negative number to specify how many loops must go by until the callback is called.
    fn handle_flight_loop(
        &mut self,
        elapsed_since_last_call: f32,
        elapsed_since_last_flight_loop: f32,
        counter: i32,
    ) -> f32;
}

/// A link to [`FlightLoopHandler`].
pub struct FlightLoopLink(Box<dyn FlightLoopHandler>);

impl FlightLoopLink {
    /// Creates a new [`FlightLoopLink`] instance.
    ///
    /// # Arguments
    /// * `value` - a pointer to the [`FlightLoopHandler`] instance.
    ///
    /// # Returns
    /// Return the flight loop link instance.
    pub fn new(value: Box<dyn FlightLoopHandler>) -> Self {
        Self(value)
    }
}

impl FlightLoopHandler for FlightLoopLink {
    fn handle_flight_loop(
        &mut self,
        elapsed_since_last_call: f32,
        elapsed_since_last_flight_loop: f32,
        counter: i32,
    ) -> f32 {
        self.0.handle_flight_loop(
            elapsed_since_last_call,
            elapsed_since_last_flight_loop,
            counter,
        )
    }
}

/// A flight loop callback handler record.
pub struct FlightLoopRecord {
    /// A flight loop identififer.
    pub id: FlightLoopId,
    /// A flight loop link to event handler.
    pub link: Box<FlightLoopLink>,
}

impl FlightLoopRecord {
    /// Creates a new flight loop handler record instance.
    ///
    /// # Arguments
    /// * `id` - a flight loop identififer.
    /// * `link` - a pointer to the flight loop link.
    ///
    /// # Return
    /// Return the new flight loop handler record instance.
    pub fn new(id: FlightLoopId, link: Box<FlightLoopLink>) -> Self {
        Self { id, link }
    }
}

impl Drop for FlightLoopRecord {
    fn drop(&mut self) {
        destroy_flight_loop(&self.id)
    }
}
