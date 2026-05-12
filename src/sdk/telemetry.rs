//! Telemetry-specific types from `scssdk_telemetry_event.h`,
//! `scssdk_telemetry_channel.h`, and `scssdk_telemetry.h`.

use std::os::raw::c_void;

use super::types::{
    ScsContext, ScsInitParamsV100, ScsNamedValue, ScsResult, ScsString, ScsTimestamp, ScsU32,
    ScsValueType,
};

/// Packs a major and minor version number into a single `u32`, as `SCS_MAKE_VERSION` does in C.
pub const fn scs_make_version(major: u32, minor: u32) -> u32 {
    (major << 16) | minor
}

pub const SCS_TELEMETRY_VERSION_1_00: u32 = scs_make_version(1, 0);
pub const SCS_TELEMETRY_VERSION_CURRENT: u32 = SCS_TELEMETRY_VERSION_1_00;

pub type ScsEvent = ScsU32;

pub const SCS_TELEMETRY_EVENT_INVALID: ScsEvent = 0;
/// Fired before any channel callbacks for the current frame.
pub const SCS_TELEMETRY_EVENT_FRAME_START: ScsEvent = 1;
/// Fired after all channel callbacks for the current frame.
pub const SCS_TELEMETRY_EVENT_FRAME_END: ScsEvent = 2;
/// The game entered a paused state (e.g. menu). Channel callbacks stop until `STARTED`.
pub const SCS_TELEMETRY_EVENT_PAUSED: ScsEvent = 3;
/// The player is now driving; channel callbacks resume.
pub const SCS_TELEMETRY_EVENT_STARTED: ScsEvent = 4;
/// Slow-changing attributes (truck config, trailer config, etc.) have changed.
pub const SCS_TELEMETRY_EVENT_CONFIGURATION: ScsEvent = 5;

/// The frame timer was restarted (e.g. after a load). Timestamps restart from zero.
pub const SCS_TELEMETRY_FRAME_START_FLAG_TIMER_RESTART: ScsU32 = 0x0000_0001;

/// Payload for `SCS_TELEMETRY_EVENT_FRAME_START`.
///
/// All times are in microseconds. Size: 32 bytes on x86 and x64.
#[repr(C)]
pub struct ScsTelemetryFrameStart {
    /// Combination of `SCS_TELEMETRY_FRAME_START_FLAG_*` flags.
    pub flags: ScsU32,
    pub _padding: ScsU32,
    /// Controls visualization; steps with rendering FPS.
    pub render_time: ScsTimestamp,
    /// Controls physics; fixed-size steps that oscillate around `render_time`.
    /// Advances even while physics is paused.
    pub simulation_time: ScsTimestamp,
    /// Same as `simulation_time` but stops when physics is paused.
    pub paused_simulation_time: ScsTimestamp,
}

#[cfg(target_arch = "x86_64")]
const _: () = {
    assert!(std::mem::size_of::<ScsTelemetryFrameStart>() == 32);
};

/// Payload for `SCS_TELEMETRY_EVENT_CONFIGURATION`.
///
/// Size: 16 bytes on x64.
#[repr(C)]
pub struct ScsTelemetryConfiguration {
    /// Which configuration group this describes (e.g. truck, trailer).
    pub id: ScsString,
    /// Null-terminated array of named attribute values for this group.
    pub attributes: *const ScsNamedValue,
}

#[cfg(target_arch = "x86_64")]
const _: () = {
    assert!(std::mem::size_of::<ScsTelemetryConfiguration>() == 16);
};

pub const SCS_TELEMETRY_CHANNEL_FLAG_NONE: ScsU32 = 0x0000_0000;
/// Deliver the value every frame, even when it has not changed.
pub const SCS_TELEMETRY_CHANNEL_FLAG_EACH_FRAME: ScsU32 = 0x0000_0001;
/// Deliver a `NULL` value pointer when the channel value is currently unavailable.
pub const SCS_TELEMETRY_CHANNEL_FLAG_NO_VALUE: ScsU32 = 0x0000_0002;

/// Called for each registered telemetry event.
///
/// `event_info` points to event-specific data (e.g. `ScsTelemetryFrameStart`).
pub type ScsEventCallback =
    unsafe extern "C" fn(event: ScsEvent, event_info: *const c_void, context: ScsContext);

/// Called for each registered telemetry channel.
///
/// `value` is `NULL` only when `SCS_TELEMETRY_CHANNEL_FLAG_NO_VALUE` was set
/// and the value is currently unavailable.
pub type ScsChannelCallback =
    unsafe extern "C" fn(name: ScsString, index: ScsU32, value: *const c_void, context: ScsContext);

/// Registers a callback for the given event. At most one callback per event.
pub type ScsRegisterForEventFn = unsafe extern "C" fn(
    event: ScsEvent,
    callback: ScsEventCallback,
    context: ScsContext,
) -> ScsResult;

/// Unregisters the callback for the given event.
pub type ScsUnregisterFromEventFn = unsafe extern "C" fn(event: ScsEvent) -> ScsResult;

/// Registers a callback for the given channel, index, and value type.
pub type ScsRegisterForChannelFn = unsafe extern "C" fn(
    name: ScsString,
    index: ScsU32,
    value_type: ScsValueType,
    flags: ScsU32,
    callback: ScsChannelCallback,
    context: ScsContext,
) -> ScsResult;

/// Unregisters the callback for the given channel, index, and value type.
pub type ScsUnregisterFromChannelFn =
    unsafe extern "C" fn(name: ScsString, index: ScsU32, value_type: ScsValueType) -> ScsResult;

/// Opaque base type for the `params` pointer received in `scs_telemetry_init`.
/// Cast to a versioned struct once the version has been confirmed.
#[repr(C)]
pub struct ScsTelemetryInitParams {
    _unused: [u8; 0],
}

/// Initialization parameters passed by the engine for telemetry API v1.00.
///
/// Size: 64 bytes on x64.
#[repr(C)]
pub struct ScsTelemetryInitParamsV100 {
    /// Game identity, version, and log function.
    pub common: ScsInitParamsV100,
    pub register_for_event: ScsRegisterForEventFn,
    pub unregister_from_event: ScsUnregisterFromEventFn,
    pub register_for_channel: ScsRegisterForChannelFn,
    pub unregister_from_channel: ScsUnregisterFromChannelFn,
}

#[cfg(target_arch = "x86_64")]
const _: () = {
    assert!(std::mem::size_of::<ScsTelemetryInitParamsV100>() == 64);
};
