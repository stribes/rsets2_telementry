use crate::callbacks::{
    on_acceleration, on_bool, on_float, on_frame_end, on_paused, on_placement, on_s32, on_started,
    on_trailer_connected, on_trailer_placement, on_trailer_wear, on_u32,
};
use crate::config::on_configuration;
use crate::log::log_error;
use crate::sdk::channels;
use crate::sdk::telemetry::{
    ScsChannelCallback, ScsTelemetryInitParamsV100, SCS_TELEMETRY_CHANNEL_FLAG_NONE,
    SCS_TELEMETRY_EVENT_CONFIGURATION, SCS_TELEMETRY_EVENT_FRAME_END, SCS_TELEMETRY_EVENT_PAUSED,
    SCS_TELEMETRY_EVENT_STARTED,
};
use crate::sdk::types::{
    ScsContext, ScsResult, ScsString, ScsValueType, SCS_RESULT_OK, SCS_U32_NIL,
    SCS_VALUE_TYPE_BOOL, SCS_VALUE_TYPE_DPLACEMENT, SCS_VALUE_TYPE_FLOAT, SCS_VALUE_TYPE_FVECTOR,
    SCS_VALUE_TYPE_S32, SCS_VALUE_TYPE_U32,
};
use crate::state::TelemetryState;

unsafe fn reg<T>(
    params: &ScsTelemetryInitParamsV100,
    name: &[u8],
    value_type: ScsValueType,
    callback: ScsChannelCallback,
    field: *mut T,
) {
    let result = (params.register_for_channel)(
        name.as_ptr() as ScsString,
        SCS_U32_NIL,
        value_type,
        SCS_TELEMETRY_CHANNEL_FLAG_NONE,
        callback,
        field as ScsContext,
    );
    if result != SCS_RESULT_OK {
        log_error!(
            "rsets2-telementry: failed to register channel '{}' ({})",
            std::str::from_utf8(name)
                .unwrap_or("?")
                .trim_end_matches('\0'),
            result
        );
    }
}

unsafe fn reg_raw(
    params: &ScsTelemetryInitParamsV100,
    name: &[u8],
    value_type: ScsValueType,
    callback: ScsChannelCallback,
) -> ScsResult {
    (params.register_for_channel)(
        name.as_ptr() as ScsString,
        SCS_U32_NIL,
        value_type,
        SCS_TELEMETRY_CHANNEL_FLAG_NONE,
        callback,
        std::ptr::null_mut(),
    )
}

pub unsafe fn register_events(params: &ScsTelemetryInitParamsV100) {
    let _ = (params.register_for_event)(
        SCS_TELEMETRY_EVENT_CONFIGURATION,
        on_configuration,
        std::ptr::null_mut(),
    );
    let _ =
        (params.register_for_event)(SCS_TELEMETRY_EVENT_PAUSED, on_paused, std::ptr::null_mut());
    let _ = (params.register_for_event)(
        SCS_TELEMETRY_EVENT_STARTED,
        on_started,
        std::ptr::null_mut(),
    );
    let _ = (params.register_for_event)(
        SCS_TELEMETRY_EVENT_FRAME_END,
        on_frame_end,
        std::ptr::null_mut(),
    );
}

pub unsafe fn register_channels(params: &ScsTelemetryInitParamsV100, state: &mut TelemetryState) {
    let g = &mut state.game;
    let t = &mut state.truck;
    let n = &mut state.navigation;

    // Game
    reg(
        params,
        channels::LOCAL_SCALE,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut g.time_scale,
    );
    reg(
        params,
        channels::GAME_TIME,
        SCS_VALUE_TYPE_U32,
        on_u32,
        &mut g.game_time,
    );
    reg(
        params,
        channels::NEXT_REST_STOP,
        SCS_VALUE_TYPE_S32,
        on_s32,
        &mut g.next_rest_stop,
    );

    // Truck (movement)
    reg(
        params,
        channels::TRUCK_SPEED,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.speed,
    );
    reg(
        params,
        channels::TRUCK_CRUISE_CONTROL,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.cruise_control_speed,
    );
    reg(
        params,
        channels::TRUCK_ENGINE_GEAR,
        SCS_VALUE_TYPE_S32,
        on_s32,
        &mut t.gear,
    );
    reg(
        params,
        channels::TRUCK_DISPLAYED_GEAR,
        SCS_VALUE_TYPE_S32,
        on_s32,
        &mut t.displayed_gear,
    );
    reg(
        params,
        channels::TRUCK_HSHIFTER_SLOT,
        SCS_VALUE_TYPE_U32,
        on_u32,
        &mut t.shifter_slot,
    );

    // Truck (engine)
    reg(
        params,
        channels::TRUCK_ENGINE_RPM,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.engine_rpm,
    );
    reg(
        params,
        channels::TRUCK_ENGINE_ENABLED,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.engine_on,
    );
    reg(
        params,
        channels::TRUCK_ELECTRIC_ENABLED,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.electric_on,
    );

    // Truck (inputs)
    reg(
        params,
        channels::TRUCK_INPUT_STEERING,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.user_steer,
    );
    reg(
        params,
        channels::TRUCK_INPUT_THROTTLE,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.user_throttle,
    );
    reg(
        params,
        channels::TRUCK_INPUT_BRAKE,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.user_brake,
    );
    reg(
        params,
        channels::TRUCK_INPUT_CLUTCH,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.user_clutch,
    );
    reg(
        params,
        channels::TRUCK_EFFECTIVE_STEERING,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.game_steer,
    );
    reg(
        params,
        channels::TRUCK_EFFECTIVE_THROTTLE,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.game_throttle,
    );
    reg(
        params,
        channels::TRUCK_EFFECTIVE_BRAKE,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.game_brake,
    );
    reg(
        params,
        channels::TRUCK_EFFECTIVE_CLUTCH,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.game_clutch,
    );

    // Truck (fuel)
    reg(
        params,
        channels::TRUCK_FUEL,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.fuel,
    );
    reg(
        params,
        channels::TRUCK_FUEL_WARNING,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.fuel_warning_on,
    );
    reg(
        params,
        channels::TRUCK_FUEL_AVG_CONSUMPTION,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.fuel_avg_consumption,
    );
    reg(
        params,
        channels::TRUCK_FUEL_RANGE,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.fuel_range,
    );

    // Truck (brakes)
    reg(
        params,
        channels::TRUCK_PARKING_BRAKE,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.park_brake_on,
    );
    reg(
        params,
        channels::TRUCK_MOTOR_BRAKE,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.motor_brake_on,
    );
    reg(
        params,
        channels::TRUCK_RETARDER_LEVEL,
        SCS_VALUE_TYPE_U32,
        on_u32,
        &mut t.retarder_brake,
    );
    reg(
        params,
        channels::TRUCK_BRAKE_AIR_PRESSURE,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.brake_air_pressure,
    );
    reg(
        params,
        channels::TRUCK_BRAKE_AIR_PRESSURE_WARNING,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.brake_air_pressure_warning_on,
    );
    reg(
        params,
        channels::TRUCK_BRAKE_AIR_PRESSURE_EMERGENCY,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.brake_air_pressure_emergency_on,
    );
    reg(
        params,
        channels::TRUCK_BRAKE_TEMPERATURE,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.brake_temperature,
    );

    // Truck (adblue)
    reg(
        params,
        channels::TRUCK_ADBLUE,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.adblue,
    );
    reg(
        params,
        channels::TRUCK_ADBLUE_WARNING,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.adblue_warning_on,
    );
    reg(
        params,
        channels::TRUCK_ADBLUE_AVG_CONSUMPTION,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.adblue_avg_consumption,
    );

    // Truck (oil)
    reg(
        params,
        channels::TRUCK_OIL_PRESSURE,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.oil_pressure,
    );
    reg(
        params,
        channels::TRUCK_OIL_PRESSURE_WARNING,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.oil_pressure_warning_on,
    );
    reg(
        params,
        channels::TRUCK_OIL_TEMPERATURE,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.oil_temperature,
    );

    // Truck (water / battery)
    reg(
        params,
        channels::TRUCK_WATER_TEMPERATURE,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.water_temperature,
    );
    reg(
        params,
        channels::TRUCK_WATER_TEMPERATURE_WARNING,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.water_temperature_warning_on,
    );
    reg(
        params,
        channels::TRUCK_BATTERY_VOLTAGE,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.battery_voltage,
    );
    reg(
        params,
        channels::TRUCK_BATTERY_VOLTAGE_WARNING,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.battery_voltage_warning_on,
    );

    // Truck (lights)
    reg(
        params,
        channels::TRUCK_DASHBOARD_BACKLIGHT,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.lights_dashboard_value,
    );
    reg(
        params,
        channels::TRUCK_LBLINKER,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.blinker_left_on,
    );
    reg(
        params,
        channels::TRUCK_RBLINKER,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.blinker_right_on,
    );
    reg(
        params,
        channels::TRUCK_LIGHT_LBLINKER,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.blinker_left_active,
    );
    reg(
        params,
        channels::TRUCK_LIGHT_RBLINKER,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.blinker_right_active,
    );
    reg(
        params,
        channels::TRUCK_LIGHT_PARKING,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.lights_parking_on,
    );
    reg(
        params,
        channels::TRUCK_LIGHT_LOW_BEAM,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.lights_beam_low_on,
    );
    reg(
        params,
        channels::TRUCK_LIGHT_HIGH_BEAM,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.lights_beam_high_on,
    );
    reg(
        params,
        channels::TRUCK_LIGHT_AUX_FRONT,
        SCS_VALUE_TYPE_U32,
        on_u32,
        &mut t.lights_aux_front,
    );
    reg(
        params,
        channels::TRUCK_LIGHT_AUX_ROOF,
        SCS_VALUE_TYPE_U32,
        on_u32,
        &mut t.lights_aux_roof,
    );
    reg(
        params,
        channels::TRUCK_LIGHT_BEACON,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.lights_beacon_on,
    );
    reg(
        params,
        channels::TRUCK_LIGHT_BRAKE,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.lights_brake_on,
    );
    reg(
        params,
        channels::TRUCK_LIGHT_REVERSE,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.lights_reverse_on,
    );
    reg(
        params,
        channels::TRUCK_WIPERS,
        SCS_VALUE_TYPE_BOOL,
        on_bool,
        &mut t.wipers_on,
    );

    // Truck (wear / odometer)
    reg(
        params,
        channels::TRUCK_WEAR_ENGINE,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.wear_engine,
    );
    reg(
        params,
        channels::TRUCK_WEAR_TRANSMISSION,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.wear_transmission,
    );
    reg(
        params,
        channels::TRUCK_WEAR_CABIN,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.wear_cabin,
    );
    reg(
        params,
        channels::TRUCK_WEAR_CHASSIS,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.wear_chassis,
    );
    reg(
        params,
        channels::TRUCK_WEAR_WHEELS,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.wear_wheels,
    );
    reg(
        params,
        channels::TRUCK_ODOMETER,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut t.odometer,
    );

    // Truck (placement and acceleration, multi-field, use STATE-accessing callbacks)
    let _ = reg_raw(
        params,
        channels::TRUCK_WORLD_PLACEMENT,
        SCS_VALUE_TYPE_DPLACEMENT,
        on_placement,
    );
    let _ = reg_raw(
        params,
        channels::TRUCK_LOCAL_ACCELERATION,
        SCS_VALUE_TYPE_FVECTOR,
        on_acceleration,
    );

    // Trailer
    reg(
        params,
        channels::TRAILER_CONNECTED,
        SCS_VALUE_TYPE_BOOL,
        on_trailer_connected,
        &mut state.trailer.attached,
    );
    reg(
        params,
        channels::TRAILER_WEAR,
        SCS_VALUE_TYPE_FLOAT,
        on_trailer_wear,
        &mut state.trailer.wear,
    );
    let _ = reg_raw(
        params,
        channels::TRAILER_WORLD_PLACEMENT,
        SCS_VALUE_TYPE_DPLACEMENT,
        on_trailer_placement,
    );

    // Navigation
    reg(
        params,
        channels::TRUCK_NAVIGATION_DISTANCE,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut n.estimated_distance,
    );
    reg(
        params,
        channels::TRUCK_NAVIGATION_TIME,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut n.estimated_time,
    );
    reg(
        params,
        channels::TRUCK_NAVIGATION_SPEED_LIMIT,
        SCS_VALUE_TYPE_FLOAT,
        on_float,
        &mut n.speed_limit,
    );
}
