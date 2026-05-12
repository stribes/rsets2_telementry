use std::fmt::Write;

/// Converts an SCS game-time value (minutes since year 0001-01-01 00:00)
/// to an ISO 8601 UTC string, e.g. "0001-01-08T21:09:00Z".
pub fn game_time_to_iso8601(minutes: u32) -> String {
    const DAYS_IN_MONTH: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let total = minutes as u64;
    let hour = (total / 60) % 24;
    let minute = total % 60;
    let total_days = total / 1440;
    let year = 1 + total_days / 365;
    let mut day_of_year = (total_days % 365) as u32;
    let mut month = 1u32;
    for &days in &DAYS_IN_MONTH {
        if day_of_year < days {
            break;
        }
        day_of_year -= days;
        month += 1;
    }
    let day = day_of_year + 1;
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:00Z",
        year, month, day, hour, minute
    )
}

#[derive(Default)]
pub struct GameState {
    pub paused: bool,
    pub time_scale: f32,
    pub game_name: String,
    pub version: String,
    pub game_time: u32,
    pub next_rest_stop: i32,
}

#[derive(Default)]
pub struct TruckState {
    // Channels
    pub speed: f32,
    pub cruise_control_speed: f32,
    pub gear: i32,
    pub displayed_gear: i32,
    pub shifter_slot: u32,
    pub engine_on: bool,
    pub electric_on: bool,
    pub engine_rpm: f32,
    pub user_steer: f32,
    pub user_throttle: f32,
    pub user_brake: f32,
    pub user_clutch: f32,
    pub game_steer: f32,
    pub game_throttle: f32,
    pub game_brake: f32,
    pub game_clutch: f32,
    pub fuel: f32,
    pub fuel_warning_on: bool,
    pub fuel_avg_consumption: f32,
    pub fuel_range: f32,
    pub park_brake_on: bool,
    pub motor_brake_on: bool,
    pub retarder_brake: u32,
    pub brake_air_pressure: f32,
    pub brake_air_pressure_warning_on: bool,
    pub brake_air_pressure_emergency_on: bool,
    pub brake_temperature: f32,
    pub adblue: f32,
    pub adblue_warning_on: bool,
    pub adblue_avg_consumption: f32,
    pub oil_pressure: f32,
    pub oil_pressure_warning_on: bool,
    pub oil_temperature: f32,
    pub water_temperature: f32,
    pub water_temperature_warning_on: bool,
    pub battery_voltage: f32,
    pub battery_voltage_warning_on: bool,
    pub lights_dashboard_value: f32,
    pub blinker_left_on: bool,
    pub blinker_right_on: bool,
    pub blinker_left_active: bool,
    pub blinker_right_active: bool,
    pub lights_parking_on: bool,
    pub lights_beam_low_on: bool,
    pub lights_beam_high_on: bool,
    pub lights_aux_front: u32,
    pub lights_aux_roof: u32,
    pub lights_beacon_on: bool,
    pub lights_brake_on: bool,
    pub lights_reverse_on: bool,
    pub wipers_on: bool,
    pub wear_engine: f32,
    pub wear_transmission: f32,
    pub wear_cabin: f32,
    pub wear_chassis: f32,
    pub wear_wheels: f32,
    pub odometer: f32,
    pub placement_x: f64,
    pub placement_y: f64,
    pub placement_z: f64,
    pub placement_heading: f32,
    pub placement_pitch: f32,
    pub placement_roll: f32,
    pub acceleration_x: f32,
    pub acceleration_y: f32,
    pub acceleration_z: f32,
    // Config
    pub id: String,
    pub make: String,
    pub model: String,
    pub fuel_capacity: f32,
    pub fuel_warning_factor: f32,
    pub adblue_capacity: f32,
    pub engine_rpm_max: f32,
    pub forward_gears: u32,
    pub reverse_gears: u32,
    pub retarder_step_count: u32,
    pub shifter_type: String,
    pub air_pressure_warning_value: f32,
    pub air_pressure_emergency_value: f32,
    pub oil_pressure_warning_value: f32,
    pub water_temperature_warning_value: f32,
    pub battery_voltage_warning_value: f32,
    pub cabin_x: f32,
    pub cabin_y: f32,
    pub cabin_z: f32,
    pub head_x: f32,
    pub head_y: f32,
    pub head_z: f32,
    pub hook_x: f32,
    pub hook_y: f32,
    pub hook_z: f32,
}

#[derive(Default)]
pub struct TrailerState {
    pub attached: bool,
    pub id: String,
    pub name: String,
    pub mass: f32,
    pub wear: f32,
    pub placement_x: f64,
    pub placement_y: f64,
    pub placement_z: f64,
    pub placement_heading: f32,
    pub placement_pitch: f32,
    pub placement_roll: f32,
}

#[derive(Default)]
pub struct NavigationState {
    pub estimated_time: f32,
    pub estimated_distance: f32,
    pub speed_limit: f32,
}

#[derive(Default)]
pub struct JobState {
    pub active: bool,
    pub income: u64,
    pub deadline_time: u32,
    pub source_city: String,
    pub source_company: String,
    pub destination_city: String,
    pub destination_company: String,
    pub cargo: String,
    pub cargo_mass: f32,
}

#[derive(Default)]
pub struct TelemetryState {
    pub game: GameState,
    pub truck: TruckState,
    pub trailer: TrailerState,
    pub job: JobState,
    pub navigation: NavigationState,
}

impl TelemetryState {
    pub fn to_json(&self) -> String {
        let mut s = String::with_capacity(4096);
        let g = &self.game;
        let t = &self.truck;
        let tr = &self.trailer;
        let j = &self.job;

        // Game
        let rest_stop_iso = if g.next_rest_stop > 0 {
            game_time_to_iso8601(g.next_rest_stop as u32)
        } else {
            "0001-01-01T00:00:00Z".to_owned()
        };
        write!(s, r#"{{"game":{{"connected":true,"gameName":{:?},"paused":{},"timeScale":{:.4},"version":{:?},"telemetryPluginVersion":{:?},"time":{:?},"nextRestStopTime":{:?}}},"#,
            g.game_name, g.paused, g.time_scale, g.version,
            env!("CARGO_PKG_VERSION"),
            game_time_to_iso8601(g.game_time), rest_stop_iso).unwrap();

        // Truck
        write!(s, r#""truck":{{"#).unwrap();
        write!(s, r#""speed":{:.4},"cruiseControlSpeed":{:.4},"cruiseControlOn":{},"gear":{},"displayedGear":{},"shifterSlot":{},"#,
            t.speed * 3.6, t.cruise_control_speed * 3.6, t.cruise_control_speed > 0.0, t.gear, t.displayed_gear, t.shifter_slot).unwrap();
        write!(
            s,
            r#""engineOn":{},"electricOn":{},"engineRpm":{:.4},"#,
            t.engine_on, t.electric_on, t.engine_rpm
        )
        .unwrap();
        write!(
            s,
            r#""userSteer":{:.4},"userThrottle":{:.4},"userBrake":{:.4},"userClutch":{:.4},"#,
            t.user_steer, t.user_throttle, t.user_brake, t.user_clutch
        )
        .unwrap();
        write!(
            s,
            r#""gameSteer":{:.4},"gameThrottle":{:.4},"gameBrake":{:.4},"gameClutch":{:.4},"#,
            t.game_steer, t.game_throttle, t.game_brake, t.game_clutch
        )
        .unwrap();
        write!(
            s,
            r#""fuel":{:.4},"fuelWarningOn":{},"fuelAverageConsumption":{:.4},"fuelRange":{:.4},"#,
            t.fuel, t.fuel_warning_on, t.fuel_avg_consumption, t.fuel_range
        )
        .unwrap();
        write!(s, r#""parkBrakeOn":{},"motorBrakeOn":{},"retarderBrake":{},"airPressure":{:.4},"airPressureWarningOn":{},"airPressureEmergencyOn":{},"brakeTemperature":{:.4},"#,
            t.park_brake_on, t.motor_brake_on, t.retarder_brake, t.brake_air_pressure,
            t.brake_air_pressure_warning_on, t.brake_air_pressure_emergency_on, t.brake_temperature).unwrap();
        write!(
            s,
            r#""adblue":{:.4},"adblueWarningOn":{},"adblueAverageConsumption":{:.4},"#,
            t.adblue, t.adblue_warning_on, t.adblue_avg_consumption
        )
        .unwrap();
        write!(
            s,
            r#""oilPressure":{:.4},"oilPressureWarningOn":{},"oilTemperature":{:.4},"#,
            t.oil_pressure, t.oil_pressure_warning_on, t.oil_temperature
        )
        .unwrap();
        write!(
            s,
            r#""waterTemperature":{:.4},"waterTemperatureWarningOn":{},"#,
            t.water_temperature, t.water_temperature_warning_on
        )
        .unwrap();
        write!(
            s,
            r#""batteryVoltage":{:.4},"batteryVoltageWarningOn":{},"#,
            t.battery_voltage, t.battery_voltage_warning_on
        )
        .unwrap();
        write!(
            s,
            r#""lightsDashboardValue":{:.4},"lightsDashboardOn":{},"#,
            t.lights_dashboard_value,
            t.lights_dashboard_value > 0.0
        )
        .unwrap();
        write!(s, r#""blinkerLeftOn":{},"blinkerRightOn":{},"blinkerLeftActive":{},"blinkerRightActive":{},"#,
            t.blinker_left_on, t.blinker_right_on, t.blinker_left_active, t.blinker_right_active).unwrap();
        write!(s, r#""lightsParkingOn":{},"lightsBeamLowOn":{},"lightsBeamHighOn":{},"lightsAuxFrontOn":{},"lightsAuxRoofOn":{},"lightsBeaconOn":{},"lightsBrakeOn":{},"lightsReverseOn":{},"wipersOn":{},"#,
            t.lights_parking_on, t.lights_beam_low_on, t.lights_beam_high_on,
            t.lights_aux_front > 0, t.lights_aux_roof > 0, t.lights_beacon_on,
            t.lights_brake_on, t.lights_reverse_on, t.wipers_on).unwrap();
        write!(s, r#""wearEngine":{:.4},"wearTransmission":{:.4},"wearCabin":{:.4},"wearChassis":{:.4},"wearWheels":{:.4},"odometer":{:.4},"#,
            t.wear_engine, t.wear_transmission, t.wear_cabin, t.wear_chassis, t.wear_wheels, t.odometer).unwrap();
        write!(s, r#""placement":{{"x":{:.4},"y":{:.4},"z":{:.4},"heading":{:.6},"pitch":{:.6},"roll":{:.6}}},"#,
            t.placement_x, t.placement_y, t.placement_z, t.placement_heading, t.placement_pitch, t.placement_roll).unwrap();
        write!(
            s,
            r#""acceleration":{{"x":{:.4},"y":{:.4},"z":{:.4}}},"#,
            t.acceleration_x, t.acceleration_y, t.acceleration_z
        )
        .unwrap();

        // Truck
        write!(
            s,
            r#""id":{:?},"make":{:?},"model":{:?},"#,
            t.id, t.make, t.model
        )
        .unwrap();
        write!(s, r#""fuelCapacity":{:.4},"fuelWarningFactor":{:.4},"adblueCapacity":{:.4},"engineRpmMax":{:.4},"#,
            t.fuel_capacity, t.fuel_warning_factor, t.adblue_capacity, t.engine_rpm_max).unwrap();
        write!(
            s,
            r#""forwardGears":{},"reverseGears":{},"retarderStepCount":{},"shifterType":{:?},"#,
            t.forward_gears, t.reverse_gears, t.retarder_step_count, t.shifter_type
        )
        .unwrap();
        write!(s, r#""airPressureWarningValue":{:.4},"airPressureEmergencyValue":{:.4},"oilPressureWarningValue":{:.4},"waterTemperatureWarningValue":{:.4},"batteryVoltageWarningValue":{:.4},"#,
            t.air_pressure_warning_value, t.air_pressure_emergency_value, t.oil_pressure_warning_value,
            t.water_temperature_warning_value, t.battery_voltage_warning_value).unwrap();
        write!(s, r#""cabin":{{"x":{:.4},"y":{:.4},"z":{:.4}}},"head":{{"x":{:.4},"y":{:.4},"z":{:.4}}},"hook":{{"x":{:.4},"y":{:.4},"z":{:.4}}}"#,
            t.cabin_x, t.cabin_y, t.cabin_z, t.head_x, t.head_y, t.head_z, t.hook_x, t.hook_y, t.hook_z).unwrap();
        s.push('}');

        // Trailer
        write!(s, r#","trailer":{{"attached":{},"id":{:?},"name":{:?},"mass":{:.4},"wear":{:.4},"placement":{{"x":{:.4},"y":{:.4},"z":{:.4},"heading":{:.6},"pitch":{:.6},"roll":{:.6}}}}}"#,
            tr.attached, tr.id, tr.name, tr.mass, tr.wear,
            tr.placement_x, tr.placement_y, tr.placement_z,
            tr.placement_heading, tr.placement_pitch, tr.placement_roll).unwrap();

        // Job
        let remaining_time = if j.deadline_time > g.game_time {
            j.deadline_time - g.game_time
        } else {
            0
        };
        write!(s, r#","job":{{"active":{},"income":{},"deadlineTime":{:?},"remainingTime":{:?},"sourceCity":{:?},"sourceCompany":{:?},"destinationCity":{:?},"destinationCompany":{:?},"cargo":{:?},"cargoMass":{:.4}}}"#,
            j.active, j.income,
            game_time_to_iso8601(j.deadline_time),
            game_time_to_iso8601(remaining_time),
            j.source_city, j.source_company, j.destination_city, j.destination_company,
            j.cargo, j.cargo_mass).unwrap();

        // Navigation
        let n = &self.navigation;
        write!(
            s,
            r#","navigation":{{"estimatedTime":{:?},"estimatedDistance":{},"speedLimit":{}}}"#,
            game_time_to_iso8601((n.estimated_time / 60.0) as u32),
            n.estimated_distance as u32,
            (n.speed_limit * 3.6) as u32
        )
        .unwrap();

        s.push('}');
        s
    }
}
