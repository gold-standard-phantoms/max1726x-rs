/// The register information in this module references:
/// https://www.analog.com/media/en/technical-documentation/data-sheets/MAX17263.pdf
/// MAX17263 datasheet
use modular_bitfield::prelude::*;
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
}
