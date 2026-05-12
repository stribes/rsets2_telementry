mod sdk;

use std::net::UdpSocket;

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

    let _params = &*(params as *const ScsTelemetryInitParamsV100);

    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
        let _ = socket.send_to(b"Hello, world!", BROADCAST_ADDR);
    }

    SCS_RESULT_OK
}

#[no_mangle]
pub unsafe extern "C" fn scs_telemetry_shutdown() {}
