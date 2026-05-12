mod log;
mod sdk;

use std::net::UdpSocket;

use log::{log_error, log_info};
use sdk::telemetry::{
    ScsTelemetryInitParams, ScsTelemetryInitParamsV100, SCS_TELEMETRY_VERSION_1_00,
};
use sdk::types::{ScsResult, ScsU32, SCS_RESULT_OK, SCS_RESULT_UNSUPPORTED};

const BROADCAST_ADDR: &str = "127.0.0.1:5555";

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

    log_info!("rsets2_telemetry: init");

    match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => {
            let _ = socket.send_to(b"Hello, world!", BROADCAST_ADDR);
        }
        Err(e) => {
            log_error!("rsets2_telemetry: failed to bind UDP socket: {e}");
        }
    }

    SCS_RESULT_OK
}

#[no_mangle]
pub unsafe extern "C" fn scs_telemetry_shutdown() {
    log_info!("rsets2_telemetry: shutdown");
}
