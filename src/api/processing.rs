pub mod error;
pub mod flight_loop;
pub mod interval;
pub mod phase;

use std::ops::DerefMut;

pub use error::ProcessingError;
pub use flight_loop::FlightLoopId;
pub use flight_loop::{FlightLoopHandler, FlightLoopLink, FlightLoopRecord};
pub use interval::FlightLoopInterval;
pub use phase::FlightLoopPhaseType;

pub type Result<T> = std::result::Result<T, ProcessingError>;

/// Returns the elapsed time since the sim started up in decimal seconds.
/// This is a wall timer. It keeps counting upward even if the sim is pasued.
///
/// XPLMGetElapsedTime is not a very good timer! It lacks precision in both its
/// data type and its source. Do not attempt to use it for timing critical
/// applications like network multiplayer.
///
/// # Returns
/// Returns the elapsed time since the sim started.
pub fn get_elapsed_time() -> f32 {
    unsafe { xplm_sys::XPLMGetElapsedTime() }
}

/// Returns a counter starting at zero for each sim cycle computed/video frame rendered.
///
/// # Returns
/// Returns a cycle number.
pub fn get_cycle_number() -> i32 {
    unsafe { xplm_sys::XPLMGetCycleNumber() }
}

/// Creates a flight loop callback and returns it's ID.
/// The flight loop callback is inited to be unscheduled.
///
/// # Arguments
/// * `phase` - the phase of the flight loop. See [`FlightLoopPhaseType`] for more details.
/// * `handler` - a [`FlightLoopHandler`] struct.
pub fn create_flight_loop<H: FlightLoopHandler>(
    phase: FlightLoopPhaseType,
    handler: H,
) -> Result<FlightLoopRecord> {
    unsafe extern "C" fn handle_flight_loop(
        elapsed_since_last_call: f32,
        elapsed_since_last_flight_loop: f32,
        counter: ::std::os::raw::c_int,
        refcon: *mut ::std::os::raw::c_void,
    ) -> f32 {
        let link = refcon as *mut FlightLoopLink;
        (*link).handle_flight_loop(
            elapsed_since_last_call,
            elapsed_since_last_flight_loop,
            counter,
        )
    }

    let mut link = Box::new(FlightLoopLink::new(Box::new(handler)));
    let link_ptr: *mut FlightLoopLink = link.deref_mut();
    let mut params = xplm_sys::XPLMCreateFlightLoop_t {
        structSize: std::mem::size_of::<xplm_sys::XPLMCreateFlightLoop_t>() as _,
        phase: phase.native(),
        callbackFunc: Some(handle_flight_loop),
        refcon: link_ptr as _,
    };

    let id = unsafe { xplm_sys::XPLMCreateFlightLoop(&mut params) };
    Ok(FlightLoopRecord::new(FlightLoopId::try_from(id)?, link))
}

/// Destroys a flight loop callback by it's ID.
/// Only call it on flight loops created with the [`create_flight_loop`] API.
///
/// # Arguments
/// * `id` - the ID of the flight loop to destroy.
pub fn destroy_flight_loop(id: &FlightLoopId) {
    unsafe { xplm_sys::XPLMDestroyFlightLoop(id.native()) };
}

/// Schedules a flight loop callback for future execution.
///
/// # Arguments
/// * `id` - the ID of the flight loop to schedule.
/// * `interval` - the interval at which to run the flight loop. See below for details.
///
/// # Details
/// If inInterval is negative, it is run in a certain number of frames based on the absolute value of the input. If the interval is positive, it is a duration in seconds.
pub fn schedule_flight_loop(id: &FlightLoopId, interval: FlightLoopInterval) {
    let (value, relation) = match interval {
        FlightLoopInterval::RelativeToCurrent(value) => (value, 1),
        FlightLoopInterval::RelativeToStart(value) => (value, 0),
    };

    unsafe { xplm_sys::XPLMScheduleFlightLoop(id.native(), value, relation) };
}
