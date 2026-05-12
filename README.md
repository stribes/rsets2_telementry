# rsets2_telementry

A Linux ETS2 / ATS telemetry plugin written in Rust. It broadcasts full telemetry data as a compact JSON packet over UDP on every game frame.

## Installation

To download the latest release, visit the [releases page](https://github.com/stribes/rsets2_telementry/releases).

Copy the `.so` into the game's `Euro Truck Simulator 2/bin/linux_x64/plugins/` folder (create it if it doesn't exist):

The game will show a one-time prompt confirming the SDK is active when it next starts.

## UDP broadcast

The plugin binds an ephemeral source port and sends one UDP packet to `127.0.0.1:5555` at the end of every rendered frame.

To receive it:

```sh
nc -u -l 5555
```

## Output format

Each packet is a single compact JSON object. The top-level keys are always present:

| Key | Description |
|-----|-------------|
| `game` | Game-level state |
| `truck` | Truck sensor readings and configuration |
| `trailer` | Trailer state |
| `job` | Active job details |
| `navigation` | Route advisor data |

### `game`

| Field | Type | Description |
|-------|------|-------------|
| `connected` | bool | Always `true` while the plugin is loaded |
| `gameName` | string | Game identifier: `"ets2"` or `"ats"` |
| `paused` | bool | `true` while the game is in a paused state (menu, map, etc.) |
| `timeScale` | float | Simulated seconds per real second (typically 19 on ETS2) |
| `version` | string | Internal game API version, e.g. `"1.7"` |
| `telemetryPluginVersion` | string | Plugin version from `Cargo.toml`, e.g. `"0.1.0"` |
| `time` | ISO 8601 date | Current absolute in-game time |
| `nextRestStopTime` | ISO 8601 date | Remaining in-game time before mandatory rest (fatigue simulation). `"0001-01-01T00:00:00Z"` when disabled or not imminent |

### `truck`

#### Movement

| Field | Type | Description |
|-------|------|-------------|
| `speed` | float | Speedometer reading in km/h (negative when reversing) |
| `cruiseControlSpeed` | float | Cruise control set speed in km/h |
| `cruiseControlOn` | bool | `true` when cruise control is active |
| `gear` | int | Physical engine gear (negative = reverse, 0 = neutral) |
| `displayedGear` | int | Gear shown on the dashboard |
| `shifterSlot` | int | H-shifter handle position (0 = no slot selected) |
| `placement` | object | World-space position and orientation (see below) |
| `acceleration` | object | Vehicle-space linear acceleration in m/s² |

#### Engine

| Field | Type | Description |
|-------|------|-------------|
| `engineOn` | bool | Engine running |
| `electricOn` | bool | Electrical system on |
| `engineRpm` | float | Current engine RPM |
| `engineRpmMax` | float | Redline RPM (from truck config) |

#### Inputs

| Field | Type | Description |
|-------|------|-------------|
| `userSteer` | float | Raw steering input `[-1, 1]` |
| `userThrottle` | float | Raw throttle input `[0, 1]` |
| `userBrake` | float | Raw brake input `[0, 1]` |
| `userClutch` | float | Raw clutch input `[0, 1]` |
| `gameSteer` | float | Effective steering after interpolation `[-1, 1]` |
| `gameThrottle` | float | Effective throttle including cruise control `[0, 1]` |
| `gameBrake` | float | Effective brake (excludes retarder/parking) `[0, 1]` |
| `gameClutch` | float | Effective clutch after auto-shift `[0, 1]` |

#### Fuel

| Field | Type | Description |
|-------|------|-------------|
| `fuel` | float | Current fuel in litres |
| `fuelCapacity` | float | Tank capacity in litres |
| `fuelWarningOn` | bool | Low fuel warning active |
| `fuelWarningFactor` | float | Warning threshold as fraction of capacity |
| `fuelAverageConsumption` | float | Average consumption in L/km |
| `fuelRange` | float | Estimated range on current fuel in km |

#### Brakes

| Field | Type | Description |
|-------|------|-------------|
| `parkBrakeOn` | bool | Parking brake engaged |
| `motorBrakeOn` | bool | Motor brake engaged |
| `retarderBrake` | int | Retarder level (0 = off, up to `retarderStepCount`) |
| `retarderStepCount` | int | Number of retarder steps (0 = no retarder fitted) |
| `airPressure` | float | Brake air tank pressure in psi |
| `airPressureWarningOn` | bool | Low air pressure warning active |
| `airPressureWarningValue` | float | Warning threshold in psi |
| `airPressureEmergencyOn` | bool | Emergency brakes active (critically low air) |
| `airPressureEmergencyValue` | float | Emergency threshold in psi |
| `brakeTemperature` | float | Brake temperature in °C |

#### Fluids

| Field | Type | Description |
|-------|------|-------------|
| `adblue` | float | AdBlue level in litres |
| `adblueCapacity` | float | AdBlue tank capacity in litres |
| `adblueWarningOn` | bool | Low AdBlue warning active |
| `adblueAverageConsumption` | float | Average AdBlue consumption in L/km |
| `oilPressure` | float | Oil pressure in psi |
| `oilPressureWarningOn` | bool | Low oil pressure warning active |
| `oilPressureWarningValue` | float | Warning threshold in psi |
| `oilTemperature` | float | Oil temperature in °C |
| `waterTemperature` | float | Coolant temperature in °C |
| `waterTemperatureWarningOn` | bool | High coolant temperature warning active |
| `waterTemperatureWarningValue` | float | Warning threshold in °C |
| `batteryVoltage` | float | Battery voltage in V |
| `batteryVoltageWarningOn` | bool | Low battery / not charging warning active |
| `batteryVoltageWarningValue` | float | Warning threshold in V |

#### Lights

| Field | Type | Description |
|-------|------|-------------|
| `blinkerLeftOn` | bool | Left blinker logically enabled |
| `blinkerRightOn` | bool | Right blinker logically enabled |
| `blinkerLeftActive` | bool | Left blinker light currently emitting |
| `blinkerRightActive` | bool | Right blinker light currently emitting |
| `lightsParkingOn` | bool | Parking lights on |
| `lightsBeamLowOn` | bool | Low beam on |
| `lightsBeamHighOn` | bool | High beam on |
| `lightsAuxFrontOn` | bool | Auxiliary front lights active |
| `lightsAuxRoofOn` | bool | Auxiliary roof lights active |
| `lightsBeaconOn` | bool | Beacon lights on |
| `lightsBrakeOn` | bool | Brake light active |
| `lightsReverseOn` | bool | Reverse light active |
| `lightsDashboardValue` | float | Dashboard backlight intensity `[0, 1]` |
| `lightsDashboardOn` | bool | Dashboard backlight on (`lightsDashboardValue > 0`) |
| `wipersOn` | bool | Wipers active |

#### Wear & odometer

| Field | Type | Description |
|-------|------|-------------|
| `wearEngine` | float | Engine wear `[0, 1]` |
| `wearTransmission` | float | Transmission wear `[0, 1]` |
| `wearCabin` | float | Cabin wear `[0, 1]` |
| `wearChassis` | float | Chassis wear `[0, 1]` |
| `wearWheels` | float | Average wheel wear `[0, 1]` |
| `odometer` | float | Odometer reading in km |

#### Truck config (from game, set when truck is loaded)

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Brand identifier, e.g. `"man"` |
| `make` | string | Localised brand name, e.g. `"MAN"` |
| `model` | string | Localised model name, e.g. `"TGX"` |
| `forwardGears` | int | Number of forward gears |
| `reverseGears` | int | Number of reverse gears |
| `shifterType` | string | `"arcade"`, `"automatic"`, `"manual"`, or `"hshifter"` |
| `cabin` | object | Cabin joint position in vehicle space `{x, y, z}` |
| `head` | object | Default head position in cabin space `{x, y, z}` |
| `hook` | object | Trailer hook position in vehicle space `{x, y, z}` |

#### Placement and acceleration objects

```json
"placement": {
  "x": 13475.5762,
  "y": 67.3606,
  "z": 14618.6211,
  "heading": 0.1851,
  "pitch": -0.0068,
  "roll": -0.0003
}
```

- `x`, `y`, `z` - world-space coordinates in metres
- `heading` - `[0, 1)` maps to `[0°, 360°)`, CCW from north
- `pitch` - `[-0.25, 0.25]` maps to `[-90°, 90°]`, positive = nose up
- `roll` - `[-0.5, 0.5]` maps to `[-180°, 180°]`

```json
"acceleration": { "x": 0.12, "y": -9.81, "z": 0.03 }
```

Vehicle-space linear acceleration in m/s².

### `trailer`

| Field | Type | Description |
|-------|------|-------------|
| `attached` | bool | Trailer currently coupled to the truck |
| `id` | string | Internal trailer identifier |
| `name` | string | Cargo accessory identifier |
| `mass` | float | Cargo mass in kg (from job data) |
| `wear` | float | Trailer chassis wear `[0, 1]` |
| `placement` | object | World-space placement (same format as truck placement) |

### `job`

| Field | Type | Description |
|-------|------|-------------|
| `active` | bool | A job is currently active |
| `income` | int | Expected reward in game currency |
| `deadlineTime` | ISO 8601 date | Absolute in-game deadline |
| `remainingTime` | ISO 8601 date | Remaining in-game time until deadline |
| `sourceCity` | string | Origin city (localised) |
| `sourceCompany` | string | Origin company (localised) |
| `destinationCity` | string | Destination city (localised) |
| `destinationCompany` | string | Destination company (localised) |
| `cargo` | string | Cargo name (localised) |
| `cargoMass` | float | Cargo mass in kg |

### `navigation`

| Field | Type | Description |
|-------|------|-------------|
| `estimatedTime` | ISO 8601 date | Time remaining to destination (as a duration from `0001-01-01`) |
| `estimatedDistance` | int | Remaining route distance in metres |
| `speedLimit` | int | Current route advisor speed limit in km/h |

## Example output

Prettified for readability, the actual UDP payload is a single compact line.

```json
{
  "game": {
    "connected": true,
    "gameName": "eut2",
    "paused": true,
    "timeScale": 1.0,
    "version": "1.18",
    "telemetryPluginVersion": "0.1.0",
    "time": "0001-01-01T10:15:00Z",
    "nextRestStopTime": "0001-01-01T10:45:00Z"
  },
  "truck": {
    "speed": 0.0,
    "cruiseControlSpeed": 0.0,
    "cruiseControlOn": false,
    "gear": 0,
    "displayedGear": 0,
    "shifterSlot": 0,
    "engineOn": false,
    "electricOn": false,
    "engineRpm": 0.0,
    "userSteer": 0.0,
    "userThrottle": 0.0,
    "userBrake": 0.0,
    "userClutch": 0.0,
    "gameSteer": 0.0,
    "gameThrottle": 0.0,
    "gameBrake": 0.0,
    "gameClutch": 0.0,
    "fuel": 0.0,
    "fuelWarningOn": false,
    "fuelAverageConsumption": 0.0,
    "fuelRange": 0.0,
    "parkBrakeOn": false,
    "motorBrakeOn": false,
    "retarderBrake": 0,
    "airPressure": 0.0,
    "airPressureWarningOn": false,
    "airPressureEmergencyOn": false,
    "brakeTemperature": 0.0,
    "adblue": 0.0,
    "adblueWarningOn": false,
    "adblueAverageConsumption": 0.0,
    "oilPressure": 0.0,
    "oilPressureWarningOn": false,
    "oilTemperature": 0.0,
    "waterTemperature": 0.0,
    "waterTemperatureWarningOn": false,
    "batteryVoltage": 0.0,
    "batteryVoltageWarningOn": false,
    "lightsDashboardValue": 0.0,
    "lightsDashboardOn": false,
    "blinkerLeftOn": false,
    "blinkerRightOn": false,
    "blinkerLeftActive": false,
    "blinkerRightActive": false,
    "lightsParkingOn": false,
    "lightsBeamLowOn": false,
    "lightsBeamHighOn": false,
    "lightsAuxFrontOn": false,
    "lightsAuxRoofOn": false,
    "lightsBeaconOn": false,
    "lightsBrakeOn": false,
    "lightsReverseOn": false,
    "wipersOn": false,
    "wearEngine": 0.0,
    "wearTransmission": 0.0,
    "wearCabin": 0.0,
    "wearChassis": 0.0,
    "wearWheels": 0.0,
    "odometer": 0.0,
    "placement": {
      "x": 0.0,
      "y": 0.0,
      "z": 0.0,
      "heading": 0.0,
      "pitch": 0.0,
      "roll": 0.0
    },
    "acceleration": { "x": 0.0, "y": 0.0, "z": 0.0 },
    "id": "mercedes",
    "make": "Mercedes-Benz",
    "model": "Actros",
    "fuelCapacity": 600.0,
    "fuelWarningFactor": 0.15,
    "adblueCapacity": 80.0,
    "engineRpmMax": 2000.0,
    "forwardGears": 12,
    "reverseGears": 4,
    "retarderStepCount": 5,
    "shifterType": "automatic",
    "airPressureWarningValue": 69.6,
    "airPressureEmergencyValue": 34.8,
    "oilPressureWarningValue": 10.15,
    "waterTemperatureWarningValue": 95.0,
    "batteryVoltageWarningValue": 23.76,
    "cabin": { "x": 0.0, "y": 1.1143, "z": -2.2729 },
    "head": { "x": -0.7479, "y": 1.6423, "z": 0.397 },
    "hook": { "x": 0.0, "y": 1.0, "z": 2.3377 }
  },
  "trailer": {
    "attached": false,
    "id": "scs_box.insulated.chassis_stwx2esii",
    "name": "",
    "mass": 15750.8994,
    "wear": 0.0,
    "placement": {
      "x": 0.0,
      "y": 0.0,
      "z": 0.0,
      "heading": 0.0,
      "pitch": 0.0,
      "roll": 0.0
    }
  },
  "job": {
    "active": true,
    "income": 5668,
    "deadlineTime": "0001-01-01T21:30:00Z",
    "remainingTime": "0001-01-01T11:15:00Z",
    "sourceCity": "Dresden",
    "sourceCompany": "Deutsche Frachth\xc3\xa4fen",
    "destinationCity": "Rostock",
    "destinationCompany": "RT Log",
    "cargo": "Goat Cheese",
    "cargoMass": 15750.8994
  },
  "navigation": {
    "estimatedTime": "0001-01-01T00:00:00Z",
    "estimatedDistance": 0,
    "speedLimit": 0
  }
}
```

## Date format

All date/time fields use a simplified ISO 8601 format in UTC:

```
0001-01-08T21:09:00Z
```

The game uses a custom calendar starting at year 1, January 1 (a Monday). There are no leap years. Time is represented internally as minutes since this epoch.

Relative durations (e.g. `remainingTime`, `nextRestStopTime`, `estimatedTime`) are expressed as if they were absolute dates from the same epoch - so `"0001-01-01T02:05:00Z"` means 2 hours and 5 minutes remaining.
