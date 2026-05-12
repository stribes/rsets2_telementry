//! Channel name constants from `scssdk_telemetry_common_channels.h`
//! and `scssdk_telemetry_truck_common_channels.h`.
//!
//! Each constant is a null-terminated byte string for use with the C API.

// Game
pub const LOCAL_SCALE: &[u8] = b"local.scale\0";
pub const GAME_TIME: &[u8] = b"game.time\0";
pub const NEXT_REST_STOP: &[u8] = b"rest.stop\0";

// Truck (movement & transmission)
pub const TRUCK_SPEED: &[u8] = b"truck.speed\0";
pub const TRUCK_CRUISE_CONTROL: &[u8] = b"truck.cruise_control\0";
pub const TRUCK_ENGINE_GEAR: &[u8] = b"truck.engine.gear\0";
pub const TRUCK_DISPLAYED_GEAR: &[u8] = b"truck.displayed.gear\0";
pub const TRUCK_HSHIFTER_SLOT: &[u8] = b"truck.hshifter.slot\0";
pub const TRUCK_WORLD_PLACEMENT: &[u8] = b"truck.world.placement\0";
pub const TRUCK_LOCAL_ACCELERATION: &[u8] = b"truck.local.acceleration.linear\0";

// Truck (engine)
pub const TRUCK_ENGINE_RPM: &[u8] = b"truck.engine.rpm\0";
pub const TRUCK_ENGINE_ENABLED: &[u8] = b"truck.engine.enabled\0";
pub const TRUCK_ELECTRIC_ENABLED: &[u8] = b"truck.electric.enabled\0";

// Truck (inputs)
pub const TRUCK_INPUT_STEERING: &[u8] = b"truck.input.steering\0";
pub const TRUCK_INPUT_THROTTLE: &[u8] = b"truck.input.throttle\0";
pub const TRUCK_INPUT_BRAKE: &[u8] = b"truck.input.brake\0";
pub const TRUCK_INPUT_CLUTCH: &[u8] = b"truck.input.clutch\0";
pub const TRUCK_EFFECTIVE_STEERING: &[u8] = b"truck.effective.steering\0";
pub const TRUCK_EFFECTIVE_THROTTLE: &[u8] = b"truck.effective.throttle\0";
pub const TRUCK_EFFECTIVE_BRAKE: &[u8] = b"truck.effective.brake\0";
pub const TRUCK_EFFECTIVE_CLUTCH: &[u8] = b"truck.effective.clutch\0";

// Truck (fuel)
pub const TRUCK_FUEL: &[u8] = b"truck.fuel.amount\0";
pub const TRUCK_FUEL_WARNING: &[u8] = b"truck.fuel.warning\0";
pub const TRUCK_FUEL_AVG_CONSUMPTION: &[u8] = b"truck.fuel.consumption.average\0";
pub const TRUCK_FUEL_RANGE: &[u8] = b"truck.fuel.range\0";

// Truck (brakes)
pub const TRUCK_PARKING_BRAKE: &[u8] = b"truck.brake.parking\0";
pub const TRUCK_MOTOR_BRAKE: &[u8] = b"truck.brake.motor\0";
pub const TRUCK_RETARDER_LEVEL: &[u8] = b"truck.brake.retarder\0";
pub const TRUCK_BRAKE_AIR_PRESSURE: &[u8] = b"truck.brake.air.pressure\0";
pub const TRUCK_BRAKE_AIR_PRESSURE_WARNING: &[u8] = b"truck.brake.air.pressure.warning\0";
pub const TRUCK_BRAKE_AIR_PRESSURE_EMERGENCY: &[u8] = b"truck.brake.air.pressure.emergency\0";
pub const TRUCK_BRAKE_TEMPERATURE: &[u8] = b"truck.brake.temperature\0";

// Truck — fluids
pub const TRUCK_ADBLUE: &[u8] = b"truck.adblue\0";
pub const TRUCK_ADBLUE_WARNING: &[u8] = b"truck.adblue.warning\0";
pub const TRUCK_ADBLUE_AVG_CONSUMPTION: &[u8] = b"truck.adblue.consumption.average\0";
pub const TRUCK_OIL_PRESSURE: &[u8] = b"truck.oil.pressure\0";
pub const TRUCK_OIL_PRESSURE_WARNING: &[u8] = b"truck.oil.pressure.warning\0";
pub const TRUCK_OIL_TEMPERATURE: &[u8] = b"truck.oil.temperature\0";
pub const TRUCK_WATER_TEMPERATURE: &[u8] = b"truck.water.temperature\0";
pub const TRUCK_WATER_TEMPERATURE_WARNING: &[u8] = b"truck.water.temperature.warning\0";
pub const TRUCK_BATTERY_VOLTAGE: &[u8] = b"truck.battery.voltage\0";
pub const TRUCK_BATTERY_VOLTAGE_WARNING: &[u8] = b"truck.battery.voltage.warning\0";

// Truck (lights)
pub const TRUCK_LBLINKER: &[u8] = b"truck.lblinker\0";
pub const TRUCK_RBLINKER: &[u8] = b"truck.rblinker\0";
pub const TRUCK_LIGHT_LBLINKER: &[u8] = b"truck.light.lblinker\0";
pub const TRUCK_LIGHT_RBLINKER: &[u8] = b"truck.light.rblinker\0";
pub const TRUCK_LIGHT_PARKING: &[u8] = b"truck.light.parking\0";
pub const TRUCK_LIGHT_LOW_BEAM: &[u8] = b"truck.light.beam.low\0";
pub const TRUCK_LIGHT_HIGH_BEAM: &[u8] = b"truck.light.beam.high\0";
pub const TRUCK_LIGHT_AUX_FRONT: &[u8] = b"truck.light.aux.front\0";
pub const TRUCK_LIGHT_AUX_ROOF: &[u8] = b"truck.light.aux.roof\0";
pub const TRUCK_LIGHT_BEACON: &[u8] = b"truck.light.beacon\0";
pub const TRUCK_LIGHT_BRAKE: &[u8] = b"truck.light.brake\0";
pub const TRUCK_LIGHT_REVERSE: &[u8] = b"truck.light.reverse\0";
pub const TRUCK_WIPERS: &[u8] = b"truck.wipers\0";
pub const TRUCK_DASHBOARD_BACKLIGHT: &[u8] = b"truck.dashboard.backlight\0";

// Truck (wear & odometer)
pub const TRUCK_WEAR_ENGINE: &[u8] = b"truck.wear.engine\0";
pub const TRUCK_WEAR_TRANSMISSION: &[u8] = b"truck.wear.transmission\0";
pub const TRUCK_WEAR_CABIN: &[u8] = b"truck.wear.cabin\0";
pub const TRUCK_WEAR_CHASSIS: &[u8] = b"truck.wear.chassis\0";
pub const TRUCK_WEAR_WHEELS: &[u8] = b"truck.wear.wheels\0";
pub const TRUCK_ODOMETER: &[u8] = b"truck.odometer\0";

// Truck (navigation)
pub const TRUCK_NAVIGATION_DISTANCE: &[u8] = b"truck.navigation.distance\0";
pub const TRUCK_NAVIGATION_TIME: &[u8] = b"truck.navigation.time\0";
pub const TRUCK_NAVIGATION_SPEED_LIMIT: &[u8] = b"truck.navigation.speed.limit\0";

// Trailer
pub const TRAILER_CONNECTED: &[u8] = b"trailer.connected\0";
pub const TRAILER_WEAR: &[u8] = b"trailer.wear.chassis\0";
pub const TRAILER_WORLD_PLACEMENT: &[u8] = b"trailer.world.placement\0";
