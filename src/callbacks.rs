use std::os::raw::c_void;

use crate::sdk::telemetry::ScsEvent;
use crate::sdk::types::{ScsContext, ScsString, ScsU32, ScsValue};

// Generic field-pointer callbacks, context is a raw pointer to the field to update.

pub unsafe extern "C" fn on_float(_: ScsString, _: ScsU32, value: *const c_void, ctx: ScsContext) {
    if value.is_null() || ctx.is_null() {
        return;
    }
    *(ctx as *mut f32) = (&(*(value as *const ScsValue)).storage.value_float).value;
}

pub unsafe extern "C" fn on_bool(_: ScsString, _: ScsU32, value: *const c_void, ctx: ScsContext) {
    if value.is_null() || ctx.is_null() {
        return;
    }
    *(ctx as *mut bool) = (&(*(value as *const ScsValue)).storage.value_bool).value != 0;
}

pub unsafe extern "C" fn on_s32(_: ScsString, _: ScsU32, value: *const c_void, ctx: ScsContext) {
    if value.is_null() || ctx.is_null() {
        return;
    }
    *(ctx as *mut i32) = (&(*(value as *const ScsValue)).storage.value_s32).value;
}

pub unsafe extern "C" fn on_u32(_: ScsString, _: ScsU32, value: *const c_void, ctx: ScsContext) {
    if value.is_null() || ctx.is_null() {
        return;
    }
    *(ctx as *mut u32) = (&(*(value as *const ScsValue)).storage.value_u32).value;
}

// Multi-field callbacks, access STATE directly because they update several fields at once.

pub unsafe extern "C" fn on_placement(
    _: ScsString,
    _: ScsU32,
    value: *const c_void,
    _: ScsContext,
) {
    if value.is_null() {
        return;
    }
    if let Some(s) = crate::STATE.as_mut() {
        let dp = &*(*(value as *const ScsValue)).storage.value_dplacement;
        s.truck.placement_x = dp.position.x;
        s.truck.placement_y = dp.position.y;
        s.truck.placement_z = dp.position.z;
        s.truck.placement_heading = dp.orientation.heading;
        s.truck.placement_pitch = dp.orientation.pitch;
        s.truck.placement_roll = dp.orientation.roll;
    }
}

pub unsafe extern "C" fn on_acceleration(
    _: ScsString,
    _: ScsU32,
    value: *const c_void,
    _: ScsContext,
) {
    if value.is_null() {
        return;
    }
    if let Some(s) = crate::STATE.as_mut() {
        let fv = &*(*(value as *const ScsValue)).storage.value_fvector;
        s.truck.acceleration_x = fv.x;
        s.truck.acceleration_y = fv.y;
        s.truck.acceleration_z = fv.z;
    }
}

pub unsafe extern "C" fn on_trailer_placement(
    _: ScsString,
    _: ScsU32,
    value: *const c_void,
    _: ScsContext,
) {
    if value.is_null() {
        return;
    }
    if let Some(s) = crate::STATE.as_mut() {
        let dp = &*(*(value as *const ScsValue)).storage.value_dplacement;
        s.trailer.placement_x = dp.position.x;
        s.trailer.placement_y = dp.position.y;
        s.trailer.placement_z = dp.position.z;
        s.trailer.placement_heading = dp.orientation.heading;
        s.trailer.placement_pitch = dp.orientation.pitch;
        s.trailer.placement_roll = dp.orientation.roll;
    }
}

pub unsafe extern "C" fn on_trailer_connected(
    _: ScsString,
    _: ScsU32,
    value: *const c_void,
    _: ScsContext,
) {
    if value.is_null() {
        return;
    }
    if let Some(s) = crate::STATE.as_mut() {
        s.trailer.attached = (&(*(value as *const ScsValue)).storage.value_bool).value != 0;
    }
}

pub unsafe extern "C" fn on_trailer_wear(
    _: ScsString,
    _: ScsU32,
    value: *const c_void,
    _: ScsContext,
) {
    if value.is_null() {
        return;
    }
    if let Some(s) = crate::STATE.as_mut() {
        s.trailer.wear = (&(*(value as *const ScsValue)).storage.value_float).value;
    }
}

// Game event callbacks

pub unsafe extern "C" fn on_paused(_: ScsEvent, _: *const c_void, _: ScsContext) {
    if let Some(s) = crate::STATE.as_mut() {
        s.game.paused = true;
    }
}

pub unsafe extern "C" fn on_started(_: ScsEvent, _: *const c_void, _: ScsContext) {
    if let Some(s) = crate::STATE.as_mut() {
        s.game.paused = false;
    }
}

pub unsafe extern "C" fn on_frame_end(_: ScsEvent, _: *const c_void, _: ScsContext) {
    if let (Some(state), Some(socket)) = (crate::STATE.as_ref(), crate::SOCKET.as_ref()) {
        let json = state.to_json();
        let _ = socket.send_to(json.as_bytes(), crate::BROADCAST_ADDR);
    }
}
