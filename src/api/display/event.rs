/// Event propagation state function.
pub enum EventState {
    /// Consume click.
    Consume = 1,
    /// Propagate click to other consumers.
    Propagate = 0,
}

impl From<EventState> for ::std::os::raw::c_int {
    fn from(value: EventState) -> Self {
        value as ::std::os::raw::c_int
    }
}
