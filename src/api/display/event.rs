/// Result returned from [`WindowHandler::mouse_click`] function.
pub enum EventState {
    /// Consume click.
    Consume,
    /// Propagate click to other consumers.
    Propagate,
}

impl From<EventState> for ::std::os::raw::c_int {
    fn from(value: EventState) -> Self {
        match value {
            EventState::Consume => 1,
            EventState::Propagate => 0,
        }
    }
}
