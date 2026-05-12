#![allow(static_mut_refs)]

mod callbacks;
mod config;
mod log;
mod registration;
mod sdk;
mod state;

use std::ffi::CStr;
use std::net::UdpSocket;

use log::{log_error, log_info};
use sdk::telemetry::{
    ScsTelemetryInitParams, ScsTelemetryInitParamsV100, SCS_TELEMETRY_VERSION_1_00,
};
use sdk::types::{ScsResult, ScsU32, SCS_RESULT_OK, SCS_RESULT_UNSUPPORTED};
use state::TelemetryState;

pub(crate) const BROADCAST_ADDR: &str = "127.0.0.1:5555";
pub(crate) static mut STATE: Option<Box<TelemetryState>> = None;
pub(crate) static mut SOCKET: Option<UdpSocket> = None;

#[no_mangle]
pub unsafe extern "C" fn scs_telemetry_init(
    version: ScsU32,
    params: *const ScsTelemetryInitParams,
) -> ScsResult {
    if version != SCS_TELEMETRY_VERSION_1_00 {
        return SCS_RESULT_UNSUPPORTED;
    }

    let params = &*(params as *const ScsTelemetryInitParamsV100);
    log::init(params.common.log);
    log_info!("rsets2-telementry: init");

    let mut state = Box::new(TelemetryState::default());
    state.game.game_name = CStr::from_ptr(params.common.game_id)
        .to_string_lossy()
        .into_owned();
    let v = params.common.game_version;
    state.game.version = format!("{}.{}", (v >> 16) & 0xFFFF, v & 0xFFFF);

    registration::register_events(params);
    registration::register_channels(params, &mut state);

    match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => SOCKET = Some(socket),
        Err(e) => log_error!("rsets2-telementry: failed to bind UDP socket: {e}"),
    }

    STATE = Some(state);
    SCS_RESULT_OK
}

#[no_mangle]
pub unsafe extern "C" fn scs_telemetry_shutdown() {
    log_info!("rsets2-telementry: shutdown");
    SOCKET = None;
    STATE = None;
}
