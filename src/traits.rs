/// Used to convert register values into physical values
pub trait RegisterResolver {
    /// Capacity in amp hours
    fn register_to_capacity(&self, register: u16) -> f64;
    /// Percentage
    fn register_to_percentage(&self, register: u16) -> f64;
    /// Voltage in volts
    fn register_to_voltage(&self, register: u16) -> f64;
    /// Current in amps
    fn register_to_current(&self, register: u16) -> f64;
    /// Temperature in degrees celsius
    fn register_to_temperature(&self, register: u16) -> f64;
    /// Resistance in ohms
    fn register_to_resistance(&self, register: u16) -> f64;
    /// Time in seconds
    fn register_to_time(&self, register: u16) -> f64;
}
pub trait Model {
    /// VCell Register
    const V_CELL: u8;
    ///Current Register
    const CURRENT: u8;
    /// Temp Register
    const TEMP: u8;
}
