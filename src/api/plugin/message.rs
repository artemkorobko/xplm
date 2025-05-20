/// The messages that are sent to your plugin by X-Plane.
pub enum Message {
    /// This message is sent to your plugin whenever the user’s plane crashes.
    PlaneCrashed,
    /// This message is sent to your plugin whenever a new plane is loaded.
    /// The parameter contains the index number of the plane being loaded.
    /// 0 indicates the user’s plane.
    PlaneLoaded(usize),
    /// This message is sent to your plugin whenever a plane is unloaded.
    /// The parameter contains the index number of the plane being unloaded.
    /// 0 indicates the user’s plane.
    PlaneUnloaded(usize),
    /// This message is sent to your plugin right after a livery is loaded for an airplane.
    /// You can use this to check the new livery (via datarefs) and react accordingly.
    /// The parameter contains the index number of the aircraft whose livery is changing.
    LiveryLoaded(usize),
    /// This messages is sent whenever the user’s plane is positioned at a new airport.
    AirportLoaded,
    /// This message is sent whenever new scenery is loaded.
    /// Use datarefs to determine the new scenery files that were loaded.
    SceneryLoaded,
    /// This message is sent whenever the user adjusts the number of X-Plane aircraft models.
    /// You must use XPLMCountPlanes to find out how many planes are now available.
    /// This message will only be sent in XP7 and higher because in XP6 the number of aircraft
    /// is not user-adjustable.
    AirplaneCountChanged,
    /// This message is sent to your plugin right before X-Plane writes its preferences file.
    /// You can use this for two purposes: to write your own preferences, and to modify
    /// any datarefs to influence preferences output. For example, if your plugin temporarily
    /// modifies saved preferences, you can put them back to their default values here
    /// to avoid having the tweaks be persisted if your plugin is not loaded on the
    /// next invocation of X-Plane.
    WillWritePrefs,
    /// Sent to your plugin right before X-Plane leaves virtual reality mode. At this time,
    /// you may want to clean up windows that are positioned in VR mode.
    ExitingVr,
    /// Sent to your plugin right before X-Plane enters virtual reality mode. At this time,
    /// any windows that are not positioned in VR mode will no longer be visible to the user.
    EnteredVr,
    /// Sent to your plugin if another plugin wants to take over AI planes.
    /// If you are a synthetic traffic provider, that probably means a plugin for an online network
    /// has connected and wants to supply aircraft flown by real humans, and you should cease
    /// to provide synthetic traffic. If, however, you are providing online traffic from
    /// real humans, you probably don’t want to disconnect, in which case you
    /// just ignore this message. The sender is the plugin ID of the plugin asking for control
    /// of the planes now. You can use it to find out who is requesting and whether you should
    /// yield to them. Synthetic traffic providers should always yield to online networks.
    ReleasePlanes,
    /// Sent to your plugin after FMOD sound banks are loaded.
    /// The parameter is the [XPLMBankID]. 0 for the master bank and 1 for the radio bank.
    FmodBankLoaded(usize),
    /// Sent to your plugin before FMOD sound banks are unloaded.
    /// Any associated resources should be cleaned up at this point.
    /// The parameter is the [XPLMBankID]. 0 for the master bank and 1 for the radio bank.
    FmodBankUnloading(usize),
    /// Sent to your plugin per-frame (at-most) when/if datarefs are added.
    /// It will include the new data ref total count, so that your plugin can keep a local cache
    /// of the total, see what’s changed and know which ones to inquire about if it cares.
    /// This message is only sent to plugins that enable the
    /// XPLM_WANTS_DATAREF_NOTIFICATIONS feature.
    DataRefsAdded(usize),
    /// A custom plugin message.
    Custom(usize, usize, *mut ::std::os::raw::c_void),
}

/// A trait which declares conversion to the message parameter.
pub trait AsMessageParam {
    /// Return the memory pointer to the message parameter.
    fn as_message_param(&self) -> *mut ::std::os::raw::c_void;
}

/// A message parameter that gets ignored when sending messages.
pub struct NoMessageParam;

impl AsMessageParam for NoMessageParam {
    fn as_message_param(&self) -> *mut std::os::raw::c_void {
        std::ptr::null_mut()
    }
}
