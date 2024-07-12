/// The register information in this module references:
/// https://www.analog.com/media/en/technical-documentation/data-sheets/MAX17263.pdf
/// MAX17263 datasheet
use modular_bitfield::prelude::*;

use crate::traits::RegisterResolver;

pub struct Max17263RegisterResolver {
    r_sense: f64,
}
impl Max17263RegisterResolver {
    /// Initialise the register resolver.
    /// * `r_sense` - The sense resistor value (ohms)
    pub fn new(r_sense: f64) -> Self {
        Self { r_sense }
    }
}

impl RegisterResolver for Max17263RegisterResolver {
    /// Capacity register to amp-hours
    /// LSb size: 5.0µVh / RSENSE. Min value: 0.0µVh. Max value: 327.675mVh / RSENSE.
    /// Notes: Equivalent to 0.5mAh with a 0.010Ω sense resistor.
    fn register_to_capacity(&self, register: u16) -> f64 {
        register as f64 * 5.0e-6 / self.r_sense
    }

    /// Percentage register to percentage
    /// LSb SIZE: 1/256%. Min value: 0.0%. Max value: 255.9961%.
    /// Notes: 1% LSb when reading only the upper byte.
    fn register_to_percentage(&self, register: u16) -> f64 {
        register as f64 / 256.0
    }

    /// Voltage register to volts
    /// LSb size: 78.125uV. Min value: 0.0V. Max value: 5.11992V.
    /// Notes: On per-cell basis.
    fn register_to_voltage(&self, register: u16) -> f64 {
        register as f64 * 78.125e-6
    }

    /// Current register to amps
    /// LSb size: 1.5625uV / RSENSE. Min value: -51.2mV / RSENSE. Max value: 51.1984mV / RSENSE.
    /// Notes: Signed 2's complement format. Equivalent to 156.25µA with a 0.010Ω sense resistor.
    fn register_to_current(&self, register: u16) -> f64 {
        (register as i16) as f64 * 1.5625e-6 / self.r_sense
    }

    /// Temperature register to degrees celsius
    /// LSb size: 1/256°C. Min value: -128°C. Max value: 127.996°C.
    /// Notes: Signed 2's complement format. 1°C LSb when reading only the upper byte.
    fn register_to_temperature(&self, register: u16) -> f64 {
        (register as i16) as f64 / 256.0
    }

    /// Resistance register to ohms
    /// LSb size: 1/4096Ω. Min value: 0Ω. Max value: 15.99976Ω.
    fn register_to_resistance(&self, register: u16) -> f64 {
        register as f64 / 4096.0
    }

    /// Time register to seconds
    /// LSb size: 5.625s. Min value: 0s. Max value: 102.3984h.
    fn register_to_time(&self, register: u16) -> f64 {
        register as f64 * 5.625
    }
}

pub struct Register;
impl Register {
    /// LEDCfg1 Register (40h) (page 29)
    /// Initial value: 0x6070
    /// The LEDCfg1 register configures the LED driver operation. If any LED activity is initiated, the MAX17263 automatically
    /// wakes up from hibernate mode into active mode.
    pub const LED_CFG_1: u8 = 0x40;
    /// VCell Register (09h) (page 22)
    /// Register Type: Voltage
    /// In multi-cell application, VCell register reports the 2.5X the voltage measured at the Cellx pin. This represents the per
    /// cell voltage of the battery pack. In single-cell application, VCell register reports the voltage measured between BATT and
    /// GND
    pub const V_CELL: u8 = 0x09;
    ///Current Register (0Ah) (page 23)
    ///Register Type: Current
    ///The IC measures the voltage across the sense resistor, and the result is stored as a two’s complement value in the
    ///Current register. Voltages outside the minimum and maximum register values are reported as the minimum or maximum
    ///value. The register value should be divided by the sense resistance to convert to amperes. The value of the sense resistor
    ///determines the resolution and the full-scale range of the current readings. Table 9 shows range and resolution values
    ///for typical sense resistances. This is for rechargeable applications. Non-rechargeable applications with long run-times
    ///should generally use higher sense resistor value.
    pub const CURRENT: u8 = 0x0A;

    /// Temp Register (08h) (page 24)
    /// Register Type: Temperature
    /// The Temp register provides the temperature measured by the thermistor or die temperature based on the Config register
    /// setting.
    pub const TEMP: u8 = 0x08;
}

/// LEDCfg1 Register (40h) (page 29)
/// Initial value: 0x6070
/// The LEDCfg1 register configures the LED driver operation. If any LED activity is initiated, the MAX17263 automatically
/// wakes up from hibernate mode into active mode.
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Default, Debug)]
pub struct LedCfg1 {
    /// NBARS: Sets the number of LED bars. After LED auto-count, this value is updated automatically.
    pub n_bars: B4,

    /// GrEn: Set GrEn = 1 to enable gray-scale for the 'remainder' LED. Otherwise, LEDs are based on proper rounding math.
    /// See the table in the datasheet for examples.
    pub gr_en: bool,

    /// LChg: Set LChg = 1 to constantly drive LEDs when battery charging (charge current > IchgTerm register) is detected.
    pub l_chg: bool,

    /// LEDMd: LED Mode. Set LEDMd = 00 to disable LEDs. Set LEDMd = 10 for direct push-button control. Set LEDMd
    /// = 01 for push-button start and timer-stop. Set LEDMd = 11 to force LEDs to turn on regardless of push-button and without
    /// any timer. LEDMd configuration effects LEDCtrl configuration.
    pub led_md: B2,

    /// AniMd: Animation Mode Control. Only applicable for LEDMd = 01 or 11. Set AniMd = 00 for normal behavior; solid bars
    /// with one gray. Set AniMd = 01 for animation to fill the bars. Set AniMd = 10 for breathing LEDs. Set AniMd = 11 for fill
    /// animation plus breathing animation.
    pub ani_md: B2,

    /// AniStep: Determines the step-size of the animation-mode operation. Larger AniStep animates faster.
    pub ani_step: B3,

    /// LEDTimer: LEDTimer sets the LED termination time according to the table in the datasheet.
    pub led_timer: B3,
}
impl defmt::Format for LedCfg1 {
    fn format(&self, f: defmt::Formatter) {
        // format the bitfields of the register
        defmt::write!(
            f,
            "NBARS: {}, GrEn: {}, LChg: {}, LEDMd: {}, AniMd: {}, AniStep: {}, LEDTimer: {}",
            self.n_bars(),
            self.gr_en(),
            self.l_chg(),
            self.led_md(),
            self.ani_md(),
            self.ani_step(),
            self.led_timer()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn led_cfg_1() {
        // Set the initial value
        let led_cfg_1 = LedCfg1::from(0x6070);
        assert_eq!(led_cfg_1.n_bars(), 0);
        assert!(led_cfg_1.gr_en());
        assert!(led_cfg_1.l_chg());
        assert_eq!(led_cfg_1.led_md(), 1);
        assert_eq!(led_cfg_1.ani_md(), 0);
        assert_eq!(led_cfg_1.ani_step(), 0);
        assert_eq!(led_cfg_1.led_timer(), 3);
    }

    #[test]
    fn test_register_to_capacity() {
        let resolver = Max17263RegisterResolver::new(0.010);

        assert_eq!(resolver.register_to_capacity(0x0000), 0.0);
        assert!(
            (resolver.register_to_capacity(0xFFFF) - 327.675e-3 / resolver.r_sense).abs() < 1e-6
        );
    }

    #[test]
    fn test_register_to_percentage() {
        let resolver = Max17263RegisterResolver::new(0.010);

        assert_eq!(resolver.register_to_percentage(0x0000), 0.0);
        assert!((resolver.register_to_percentage(0xFFFF) - 255.9961).abs() < 1e-4);
    }

    #[test]
    fn test_register_to_voltage() {
        let resolver = Max17263RegisterResolver::new(0.010);

        assert_eq!(resolver.register_to_voltage(0x0000), 0.0);
        assert!((resolver.register_to_voltage(0xFFFF) - 5.11992).abs() < 1e-5);
    }

    #[test]
    fn test_register_to_current() {
        let resolver = Max17263RegisterResolver::new(0.010);

        assert!(
            (resolver.register_to_current(i16::MIN as u16) + 51.2e-3 / resolver.r_sense).abs()
                < 1e-5
        );
        assert!(
            (resolver.register_to_current(i16::MAX as u16) - 51.1984e-3 / resolver.r_sense).abs()
                < 1e-5
        );
    }

    #[test]
    fn test_register_to_temperature() {
        let resolver = Max17263RegisterResolver::new(0.010);

        assert_eq!(resolver.register_to_temperature(0x0000), 0.0);
        assert!((resolver.register_to_temperature(0x7FFF) - 127.996).abs() < 1e-3);
        assert!((resolver.register_to_temperature(0x8000) - -128.0).abs() < 1e-3);
    }

    #[test]
    fn test_register_to_resistance() {
        let resolver = Max17263RegisterResolver::new(0.010);

        assert_eq!(resolver.register_to_resistance(0x0000), 0.0);
        assert!((resolver.register_to_resistance(0xFFFF) - 15.99976).abs() < 1e-5);
    }

    #[test]
    fn test_register_to_time() {
        let resolver = Max17263RegisterResolver::new(0.010);

        assert_eq!(resolver.register_to_time(0x0000), 0.0);
        assert!((resolver.register_to_time(0xFFFF) - 102.3984 * 3600.0).abs() < 1.0);
    }
}
