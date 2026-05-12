use std::ffi::CString;

use crate::sdk::types::{ScsLogFn, ScsLogType};

static mut LOG_FN: Option<ScsLogFn> = None;

pub unsafe fn init(log_fn: ScsLogFn) {
    LOG_FN = Some(log_fn);
}

pub unsafe fn write(level: ScsLogType, message: &str) {
    if let Some(log_fn) = LOG_FN {
        if let Ok(cstring) = CString::new(message) {
            log_fn(level, cstring.as_ptr());
        }
    }
}

macro_rules! log_info {
    ($($arg:tt)*) => {
        crate::log::write($crate::sdk::types::SCS_LOG_TYPE_MESSAGE, &format!($($arg)*))
    };
}

macro_rules! log_error {
    ($($arg:tt)*) => {
        crate::log::write($crate::sdk::types::SCS_LOG_TYPE_ERROR, &format!($($arg)*))
    };
}

pub(crate) use {log_error, log_info};
