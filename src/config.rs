use std::ffi::CStr;
use std::os::raw::c_void;

use crate::sdk::telemetry::{ScsEvent, ScsTelemetryConfiguration};
use crate::sdk::types::{
    ScsContext, ScsNamedValue, ScsValue, ScsValueFvector, SCS_VALUE_TYPE_FLOAT,
    SCS_VALUE_TYPE_FVECTOR, SCS_VALUE_TYPE_STRING, SCS_VALUE_TYPE_U32, SCS_VALUE_TYPE_U64,
};

unsafe fn attr_str(v: &ScsValue) -> Option<String> {
    if v.value_type != SCS_VALUE_TYPE_STRING {
        return None;
    }
    let ptr = (&v.storage.value_string).value;
    if ptr.is_null() {
        return None;
    }
    Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
}

unsafe fn attr_float(v: &ScsValue) -> Option<f32> {
    if v.value_type != SCS_VALUE_TYPE_FLOAT {
        return None;
    }
    Some((&v.storage.value_float).value)
}

unsafe fn attr_u32(v: &ScsValue) -> Option<u32> {
    if v.value_type != SCS_VALUE_TYPE_U32 {
        return None;
    }
    Some((&v.storage.value_u32).value)
}

unsafe fn attr_u64(v: &ScsValue) -> Option<u64> {
    if v.value_type != SCS_VALUE_TYPE_U64 {
        return None;
    }
    Some((&v.storage.value_u64).value)
}

unsafe fn attr_fvec(v: &ScsValue) -> Option<(f32, f32, f32)> {
    if v.value_type != SCS_VALUE_TYPE_FVECTOR {
        return None;
    }
    let fv = &*(&v.storage.value_fvector as *const _ as *const ScsValueFvector);
    Some((fv.x, fv.y, fv.z))
}

unsafe fn iter_attrs(attributes: *const ScsNamedValue, mut f: impl FnMut(&str, &ScsValue)) {
    let mut ptr = attributes;
    loop {
        let attr = &*ptr;
        if attr.name.is_null() {
            break;
        }
        if let Ok(name) = CStr::from_ptr(attr.name).to_str() {
            f(name, &attr.value);
        }
        ptr = ptr.add(1);
    }
}

pub unsafe extern "C" fn on_configuration(_: ScsEvent, event_info: *const c_void, _: ScsContext) {
    if event_info.is_null() {
        return;
    }
    let config = &*(event_info as *const ScsTelemetryConfiguration);
    if config.id.is_null() || config.attributes.is_null() {
        return;
    }
    let Ok(config_id) = CStr::from_ptr(config.id).to_str() else {
        return;
    };
    let Some(s) = crate::STATE.as_mut() else {
        return;
    };

    match config_id {
        "truck" => {
            let t = &mut s.truck;
            iter_attrs(config.attributes, |name, v| match name {
                "brand_id" => {
                    if let Some(x) = attr_str(v) {
                        t.id = x;
                    }
                }
                "brand" => {
                    if let Some(x) = attr_str(v) {
                        t.make = x;
                    }
                }
                "name" => {
                    if let Some(x) = attr_str(v) {
                        t.model = x;
                    }
                }
                "fuel.capacity" => {
                    if let Some(x) = attr_float(v) {
                        t.fuel_capacity = x;
                    }
                }
                "fuel.warning.factor" => {
                    if let Some(x) = attr_float(v) {
                        t.fuel_warning_factor = x;
                    }
                }
                "adblue.capacity" => {
                    if let Some(x) = attr_float(v) {
                        t.adblue_capacity = x;
                    }
                }
                "brake.air.pressure.warning" => {
                    if let Some(x) = attr_float(v) {
                        t.air_pressure_warning_value = x;
                    }
                }
                "brake.air.pressure.emergency" => {
                    if let Some(x) = attr_float(v) {
                        t.air_pressure_emergency_value = x;
                    }
                }
                "oil.pressure.warning" => {
                    if let Some(x) = attr_float(v) {
                        t.oil_pressure_warning_value = x;
                    }
                }
                "water.temperature.warning" => {
                    if let Some(x) = attr_float(v) {
                        t.water_temperature_warning_value = x;
                    }
                }
                "battery.voltage.warning" => {
                    if let Some(x) = attr_float(v) {
                        t.battery_voltage_warning_value = x;
                    }
                }
                "rpm.limit" => {
                    if let Some(x) = attr_float(v) {
                        t.engine_rpm_max = x;
                    }
                }
                "gears.forward" => {
                    if let Some(x) = attr_u32(v) {
                        t.forward_gears = x;
                    }
                }
                "gears.reverse" => {
                    if let Some(x) = attr_u32(v) {
                        t.reverse_gears = x;
                    }
                }
                "retarder.steps" => {
                    if let Some(x) = attr_u32(v) {
                        t.retarder_step_count = x;
                    }
                }
                "cabin.position" => {
                    if let Some((x, y, z)) = attr_fvec(v) {
                        t.cabin_x = x;
                        t.cabin_y = y;
                        t.cabin_z = z;
                    }
                }
                "head.position" => {
                    if let Some((x, y, z)) = attr_fvec(v) {
                        t.head_x = x;
                        t.head_y = y;
                        t.head_z = z;
                    }
                }
                "hook.position" => {
                    if let Some((x, y, z)) = attr_fvec(v) {
                        t.hook_x = x;
                        t.hook_y = y;
                        t.hook_z = z;
                    }
                }
                _ => {}
            });
        }
        "controls" => {
            let t = &mut s.truck;
            iter_attrs(config.attributes, |name, v| {
                if name == "shifter.type" {
                    if let Some(x) = attr_str(v) {
                        t.shifter_type = x;
                    }
                }
            });
        }
        "trailer" => {
            let tr = &mut s.trailer;
            iter_attrs(config.attributes, |name, v| match name {
                "id" => {
                    if let Some(x) = attr_str(v) {
                        tr.id = x;
                    }
                }
                "cargo.accessory.id" => {
                    if let Some(x) = attr_str(v) {
                        tr.name = x;
                    }
                }
                _ => {}
            });
        }
        "job" => {
            let j = &mut s.job;
            if (*config.attributes).name.is_null() {
                *j = Default::default();
                s.trailer.mass = 0.0;
                return;
            }
            j.active = true;
            iter_attrs(config.attributes, |name, v| match name {
                "income" => {
                    if let Some(x) = attr_u64(v) {
                        j.income = x;
                    }
                }
                "delivery.time" => {
                    if let Some(x) = attr_u32(v) {
                        j.deadline_time = x;
                    }
                }
                "source.city" => {
                    if let Some(x) = attr_str(v) {
                        j.source_city = x;
                    }
                }
                "source.company" => {
                    if let Some(x) = attr_str(v) {
                        j.source_company = x;
                    }
                }
                "destination.city" => {
                    if let Some(x) = attr_str(v) {
                        j.destination_city = x;
                    }
                }
                "destination.company" => {
                    if let Some(x) = attr_str(v) {
                        j.destination_company = x;
                    }
                }
                "cargo" => {
                    if let Some(x) = attr_str(v) {
                        j.cargo = x;
                    }
                }
                "cargo.mass" => {
                    if let Some(x) = attr_float(v) {
                        j.cargo_mass = x;
                        s.trailer.mass = x;
                    }
                }
                _ => {}
            });
        }
        _ => {}
    }
}
