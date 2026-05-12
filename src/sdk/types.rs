//! Primitive types, value structs, and common init params from `scssdk.h` and `scssdk_value.h`.

use std::os::raw::{c_char, c_void};

pub type ScsU8 = u8;
pub type ScsU16 = u16;
pub type ScsS32 = i32;
pub type ScsU32 = u32;
pub type ScsU64 = u64;
pub type ScsFloat = f32;
pub type ScsDouble = f64;

/// Null-terminated UTF-8 string pointer. Never NULL per SDK contract.
pub type ScsString = *const c_char;

/// Opaque context pointer passed through to callbacks, defined by the caller.
pub type ScsContext = *mut c_void;

/// A timestamp in microseconds.
pub type ScsTimestamp = ScsU64;

/// Return code returned by most SDK functions.
pub type ScsResult = ScsS32;

/// Severity level for messages written to the game log.
pub type ScsLogType = ScsS32;

/// Sentinel index value meaning "not present" or "not applicable".
pub const SCS_U32_NIL: ScsU32 = u32::MAX;

pub const SCS_RESULT_OK: ScsResult = 0;
pub const SCS_RESULT_UNSUPPORTED: ScsResult = -1;
pub const SCS_RESULT_INVALID_PARAMETER: ScsResult = -2;
pub const SCS_RESULT_ALREADY_REGISTERED: ScsResult = -3;
pub const SCS_RESULT_NOT_FOUND: ScsResult = -4;
pub const SCS_RESULT_UNSUPPORTED_TYPE: ScsResult = -5;
pub const SCS_RESULT_NOT_NOW: ScsResult = -6;
pub const SCS_RESULT_GENERIC_ERROR: ScsResult = -7;

pub const SCS_LOG_TYPE_MESSAGE: ScsLogType = 0;
pub const SCS_LOG_TYPE_WARNING: ScsLogType = 1;
pub const SCS_LOG_TYPE_ERROR: ScsLogType = 2;

pub type ScsValueType = ScsU32;

pub const SCS_VALUE_TYPE_INVALID: ScsValueType = 0;
pub const SCS_VALUE_TYPE_BOOL: ScsValueType = 1;
pub const SCS_VALUE_TYPE_S32: ScsValueType = 2;
pub const SCS_VALUE_TYPE_U32: ScsValueType = 3;
pub const SCS_VALUE_TYPE_U64: ScsValueType = 4;
pub const SCS_VALUE_TYPE_FLOAT: ScsValueType = 5;
pub const SCS_VALUE_TYPE_DOUBLE: ScsValueType = 6;
pub const SCS_VALUE_TYPE_FVECTOR: ScsValueType = 7;
pub const SCS_VALUE_TYPE_DVECTOR: ScsValueType = 8;
pub const SCS_VALUE_TYPE_EULER: ScsValueType = 9;
pub const SCS_VALUE_TYPE_FPLACEMENT: ScsValueType = 10;
pub const SCS_VALUE_TYPE_DPLACEMENT: ScsValueType = 11;
pub const SCS_VALUE_TYPE_STRING: ScsValueType = 12;
pub const SCS_VALUE_TYPE_LAST: ScsValueType = SCS_VALUE_TYPE_STRING;

#[repr(C)]
pub struct ScsValueBool {
    pub value: ScsU8,
}

#[repr(C)]
pub struct ScsValueS32 {
    pub value: ScsS32,
}

#[repr(C)]
pub struct ScsValueU32 {
    pub value: ScsU32,
}

#[repr(C)]
pub struct ScsValueU64 {
    pub value: ScsU64,
}

#[repr(C)]
pub struct ScsValueFloat {
    pub value: ScsFloat,
}

#[repr(C)]
pub struct ScsValueDouble {
    pub value: ScsDouble,
}

#[repr(C)]
pub struct ScsValueString {
    pub value: ScsString,
}

/// 3-component float vector. In local space: X right, Y up, Z backwards.
/// In world space: X east, Y up, Z south.
#[repr(C)]
pub struct ScsValueFvector {
    pub x: ScsFloat,
    pub y: ScsFloat,
    pub z: ScsFloat,
}

/// 3-component double vector. Same axis conventions as `ScsValueFvector`.
#[repr(C)]
pub struct ScsValueDvector {
    pub x: ScsDouble,
    pub y: ScsDouble,
    pub z: ScsDouble,
}

/// Object orientation in unit-range Euler angles.
///
/// - `heading`: `[0, 1)` maps to `[0°, 360°)`, measured CCW from north when viewed from above.
/// - `pitch`: `[-0.25, 0.25]` maps to `[-90°, 90°]`, positive is up.
/// - `roll`: `[-0.5, 0.5]` maps to `[-180°, 180°]`, measured CCW along the roll axis.
#[repr(C)]
pub struct ScsValueEuler {
    pub heading: ScsFloat,
    pub pitch: ScsFloat,
    pub roll: ScsFloat,
}

/// Combined float position and orientation.
#[repr(C)]
pub struct ScsValueFplacement {
    pub position: ScsValueFvector,
    pub orientation: ScsValueEuler,
}

/// Combined double position and orientation.
#[repr(C)]
pub struct ScsValueDplacement {
    pub position: ScsValueDvector,
    pub orientation: ScsValueEuler,
    pub _padding: ScsU32,
}

/// Tagged union holding any telemetry value.
///
/// Read `value_type` first, then access the matching field in `storage`.
/// Size: 48 bytes on x86 and x64.
#[repr(C)]
pub struct ScsValue {
    pub value_type: ScsValueType,
    pub _padding: ScsU32,
    pub storage: ScsValueStorage,
}

/// The raw storage union inside `ScsValue`.
///
/// Only the field that matches `ScsValue::value_type` is valid.
#[repr(C)]
pub union ScsValueStorage {
    pub value_bool: std::mem::ManuallyDrop<ScsValueBool>,
    pub value_s32: std::mem::ManuallyDrop<ScsValueS32>,
    pub value_u32: std::mem::ManuallyDrop<ScsValueU32>,
    pub value_u64: std::mem::ManuallyDrop<ScsValueU64>,
    pub value_float: std::mem::ManuallyDrop<ScsValueFloat>,
    pub value_double: std::mem::ManuallyDrop<ScsValueDouble>,
    pub value_fvector: std::mem::ManuallyDrop<ScsValueFvector>,
    pub value_dvector: std::mem::ManuallyDrop<ScsValueDvector>,
    pub value_euler: std::mem::ManuallyDrop<ScsValueEuler>,
    pub value_fplacement: std::mem::ManuallyDrop<ScsValueFplacement>,
    pub value_dplacement: std::mem::ManuallyDrop<ScsValueDplacement>,
    pub value_string: std::mem::ManuallyDrop<ScsValueString>,
}

/// A named attribute value, used in configuration events.
///
/// Size: 64 bytes on x64.
#[repr(C)]
pub struct ScsNamedValue {
    /// Attribute name (ASCII subset of UTF-8).
    pub name: ScsString,
    /// Zero-based index for array attributes; `SCS_U32_NIL` for scalar attributes.
    pub index: ScsU32,
    pub _padding: ScsU32,
    pub value: ScsValue,
}

#[cfg(target_arch = "x86_64")]
const _: () = {
    assert!(std::mem::size_of::<ScsValue>() == 48);
    assert!(std::mem::size_of::<ScsNamedValue>() == 64);
};

/// Writes a message to the game log.
pub type ScsLogFn = unsafe extern "C" fn(log_type: ScsLogType, message: ScsString);

/// Common initialization parameters shared by all SCS SDK APIs (v1.00).
///
/// Size: 32 bytes on x64.
#[repr(C)]
pub struct ScsInitParamsV100 {
    /// Human-readable game name (UTF-8). Never NULL.
    pub game_name: ScsString,
    /// Machine-readable game identifier (lowercase ASCII + digits + underscores). Never NULL.
    pub game_id: ScsString,
    /// Game version for this specific API, not the patch version.
    pub game_version: ScsU32,
    pub _padding: ScsU32,
    /// Writes a line to the in-game log. Never NULL.
    pub log: ScsLogFn,
}

#[cfg(target_arch = "x86_64")]
const _: () = {
    assert!(std::mem::size_of::<ScsInitParamsV100>() == 32);
};
