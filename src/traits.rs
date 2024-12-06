/// Trait for bidirectional conversion between register values and physical measurements
pub trait RegisterResolver {
    /// Converts register value to battery capacity in amp-hours (Ah)
    fn register_to_capacity(&self, register: u16) -> f64;
    /// Converts battery capacity in amp-hours (Ah) to register value
    fn capacity_to_register(&self, capacity: f64) -> u16;

    /// Converts register value to state of charge percentage (0-100%)
    fn register_to_percentage(&self, register: u16) -> f64;
    /// Converts state of charge percentage (0-100%) to register value
    fn percentage_to_register(&self, percentage: f64) -> u16;

    /// Converts register value to voltage in volts (V)
    fn register_to_voltage(&self, register: u16) -> f64;
    /// Converts voltage in volts (V) to register value
    fn voltage_to_register(&self, voltage: f64) -> u16;

    /// Converts register value to current in amperes (A)
    fn register_to_current(&self, register: u16) -> f64;
    /// Converts current in amperes (A) to register value
    fn current_to_register(&self, current: f64) -> u16;

    /// Converts register value to temperature in degrees Celsius (°C)
    fn register_to_temperature(&self, register: u16) -> f64;
    /// Converts temperature in degrees Celsius (°C) to register value
    fn temperature_to_register(&self, temperature: f64) -> u16;

    /// Converts register value to resistance in ohms (Ω)
    fn register_to_resistance(&self, register: u16) -> f64;
    /// Converts resistance in ohms (Ω) to register value
    fn resistance_to_register(&self, resistance: f64) -> u16;

    /// Converts register value to time duration in seconds (s)
    fn register_to_time(&self, register: u16) -> f64;
    /// Converts time duration in seconds (s) to register value
    fn time_to_register(&self, seconds: f64) -> u16;
}

pub trait Model {
    /// VCell Register
    const V_CELL: u8;
    /// Current Register
    const CURRENT: u8;
    /// Temp Register
    const TEMP: u8;
}

pub trait BitField {
    const REGISTER: u8;
}
