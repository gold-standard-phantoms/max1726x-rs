/// The register infomation in this module references:
/// MAX1726x ModelGauge m5 EZ User Guide UG6597; Rev 3; 11/19
/// https://www.analog.com/media/en/technical-documentation/user-guides/max1726x-modelgauge-m5-ez-user-guide.pdf
use modular_bitfield::prelude::*;

use crate::traits::BitField;

pub struct Register;
impl Register {
    /// Status register (00h) (Page 32)
    /// Register Type: Special
    /// Initial Value: 0x0002 (change to 0x8082 immediately after POR)
    /// The Status register maintains all flags related to alert thresholds and battery insertion or
    /// removal. Table 14 shows the Status register format
    pub const STATUS: u8 = 0x00;
    /// FStat Register (3Dh) (page 39)
    /// Register Type: Special
    /// The FStat register is a read-only register that monitors the status of the ModelGauge m5
    /// algorithm. Table 20 is the FStat register format.
    pub const F_STAT: u8 = 0x3D;
    /// HibCfg Register (BAh)
    /// Register Type: Special
    /// Initial Value: 0x870C
    /// The HibCfg register controls hibernate mode functionality. The MAX1726x enters and exits
    /// hibernate when the battery current is less than approximately C/100. While in hibernate mode,
    /// the MAX1726x reduces its operating current to 5μA by reducing ADC sampling to once every
    /// 5.625s. Table 24 shows the register format
    pub const HIB_CFG: u8 = 0xBA;
    /// Soft-Wakeup (Command Register 60h) (page 42)
    /// Register Type: Special
    /// To wake and exit hibernate:
    /// 1. Write HibCfg = 0x0000.
    /// 2. Soft-Wakeup Command. Write Command Register (60h) to 0x0090.
    /// 3. Clear Command. Write Command Register (60h) to 0x0000.
    /// 4. Eventually restore HibCfg to again allow automatic hibernate decisions.
    pub const SOFT_WAKEUP: u8 = 0x60;

    /// DesignCap Register (18h) (page 29)
    /// Register Type: Capacity
    /// The DesignCap register holds the expected capacity of the cell. This value is used to determine
    /// the age and health of the cell by comparing against the measured present cell capacity.
    /// mAh
    pub const DESIGN_CAP: u8 = 0x18;

    /// IChgTerm Register (1Eh) (page 29)
    /// Register Type: Current
    /// Initial Value: 0x0640 (250mA on 10mΩ)
    /// The IChgTerm register allows the device to detect when a charge cycle of the cell has
    /// completed. IChgTerm should be programmed to the exact charge termination current used in
    /// the application. The device detects end of charge if all the following conditions are met:
    /// • VFSOC Register > FullSOCThr Register
    /// • AND IChgTerm x 0.125 < Current Register < IChgTerm x 1.25
    /// • AND IChgTerm x 0.125 < AvgCurrent Register < IChgTerm x 1.25
    /// See the End-of-Charge Detection section for more details.
    pub const I_CHG_TERM: u8 = 0x1E;

    /// VEmpty Register (3Ah) (page 28)
    /// Initial Value: 0xA561 (3.3V / 3.88V)
    /// The VEmpty register sets thresholds related to empty detection during operation. Table 11
    /// shows the register format.
    pub const V_EMPTY: u8 = 0x3A;

    /// ModelCfg Register (DBh) (page 29)
    /// Register Type: Special
    /// Initial value: 0x8400
    /// The ModelCFG register controls basic options of the EZ algorithm. Table 12 shows the register
    /// format.
    pub const MODEL_CFG: u8 = 0xDB;
}

pub struct OutputRegister;
impl OutputRegister {
    /// RepCap Register (05h) (page 31)
    /// Register Type: Capacity
    /// RepCap or reported remaining capacity in mAh. This register is protected from making sudden
    /// jumps during load changes.
    pub const REP_CAP: u8 = 0x05;

    /// RepSOC Register (06h)
    /// Register Type: Percentage
    /// RepSOC is the reported state-of-charge percentage output for use by the application GUI.
    pub const REP_SOC: u8 = 0x06;

    /// TTE Register (11h)
    /// Register Type: Time
    /// The TTE register holds the estimated time to empty for the application under present
    /// temperature and load conditions. The TTE value is determined by relating AvCap with
    /// AvgCurrent.
    /// The corresponding AvgCurrent filtering gives a delay in TTE, but provides more stable results.
    /// The LSB of the TTE register is 5.625s.
    pub const TTE: u8 = 0x11;
}

defmt::bitflags! {
    /// Status register (00h) (Page 32)
    /// Register Type: Special
    /// Initial Value: 0x0002 (change to 0x8082 immediately after POR)
    /// The Status register maintains all flags related to alert thresholds and battery insertion or
    /// removal. Table 14 shows the Status register format
    pub struct Status: u16 {
        /// POR (Power-On Reset): This bit is set to 1 when the device detects that a software or
        /// hardware POR event has occurred. This bit must be cleared by system software to detect the
        /// next POR event. POR is set to 1 at power-up.
        const POR = 1 << 1;

        /// Imn and Imx (Minimum/Maximum Current Alert Threshold Exceeded): These bits are set to
        /// a 1 whenever a Current register reading is below (Imn) or above (Imx) the IAlrtTh thresholds.
        /// These bits may or may not need to be cleared by system software to detect the next event. See
        /// the Config.IS bit description. Imn and Imx are cleared to 0 at power-up.
        const IMN = 1 << 2;
        const IMX = 1 << 6;

        /// Vmn and Vmx (Minimum/Maximum Voltage Alert Threshold Exceeded): These bits are set
        /// to a 1 whenever a VCell register reading is below (Vmn) or above (Vmx) the VAlrtTh thresholds.
        /// These bits may or may not need to be cleared by system software to detect the next event. See
        /// the Config.VS bit description. Vmn and Vmx are cleared to 0 at power-up.
        const VMN = 1 << 8;
        const VMX = 1 << 12;

        /// Tmn and Tmx (Minimum/Maximum Temperature Alert Threshold Exceeded): These bits
        /// are set to a 1 whenever a Temperature register reading is below (Tmn) or above (Tmx) the
        /// TAlrtTh thresholds. These bits may or may not need to be cleared by system software to detect
        /// the next event. See the Config.TS bit description. Tmn and Tmx are cleared to 0 at power-up.
        const TMN = 1 << 9;
        const TMX = 1 << 13;

        /// Smn and Smx (Minimum/Maximum SOC Alert Threshold Exceeded): These bits set to 1
        /// when the SOC is below (Smn) or above (Smx) the SAlrtTh thresholds. These bits might or might
        /// not need to be cleared by system software to detect the next event. See the Config.SS
        /// description. Smn and Smx are cleared to 0 at power-up.
        const SMN = 1 << 10;
        const SMX = 1 << 14;

        /// Bst (Battery Status): This bit is useful when the IC is used in a host-side application. This bit is
        /// set to 0 when a battery is present in the system and set to 1 when the battery is absent. Bst is
        /// set to 0 at power-up.
        const BST = 1 << 3;

        /// dSOCi (State of Charge 1% Change Alert): This bit is set to 1 when the RepSOC register
        /// crosses an integer percentage boundary such as 50.0%, 51.0%, etc. The bit must be cleared by
        /// host software. dSOCi is set to 1 at power-up.
        const D_SOC_I = 1 << 7;

        /// Bi (Battery Insertion): This bit is useful when the IC is used in a host-side application. This bit
        /// is set to 1 when the device detects that a battery has been inserted into the system by
        /// monitoring the TH pin. This bit must be cleared by system software to detect the next insertion
        /// event. Bi is set to 0 at power-up.
        const BI = 1 << 11;

        /// Br (Battery Removal): This bit is useful when the IC is used in a host-side application. Br is set
        /// to 1 when the system detects that a battery has been removed from the system. This bit must
        /// be cleared by system software to detect the next removal event. Br is set to 1 at power-up.
        const BR = 1 << 15;
    }
}

defmt::bitflags! {
    /// FStat Register (3Dh) (page 39)
    /// Register Type: Special
    /// The FStat register is a read-only register that monitors the status of the ModelGauge m5
    /// algorithm. Table 20 is the FStat register format.
    pub struct FStat:u16{
        /// DNR: Data Not Ready. This bit is set to 1 at cell insertion and remains set until the output
        /// registers have been updated. Afterward, the IC clears this bit, indicating the fuel gauge
        /// calculations are up to date. This takes 710ms from power-up.
        const DNR = 1;

        /// FQ: Full Qualified. This bit is set when all charge termination conditions have been met. See the
        /// End-of-Charge Detection section for details.
        const FQ = 1 << 7;

        /// EDet: Empty Detection. This bit is set to 1 when the IC detects that the cell empty point has
        /// been reached. This bit is reset to 0 when the cell voltage rises above the recovery threshold.
        /// See the VEmpty register for details.
        const E_DET = 1 << 8;

        /// RelDt: Relaxed Cell Detection. This bit is set to 1 when the ModelGauge m5 algorithm detects
        /// that the cell is in a fully relaxed state. This bit is cleared to 0 when a current greater than the
        /// Load threshold is detected. See Figure 12.
        const REL_DT = 1 << 9;

        /// RelDt2: Long Relaxation. This bit is set to 1 when the ModelGauge m5 algorithm detects that
        /// the cell has been relaxed for a period of 48 to 96 minutes or longer. This bit is cleared to 0 when
        /// the cell is no longer in a relaxed state. See Figure 12.
        const REL_DT2 = 1 << 6;
    }
}

/// HibCfg Register (BAh) (page 41)
/// Register Type: Special
/// Initial Value: 0x870C
/// The HibCfg register controls hibernate mode functionality. The MAX1726x enters and exits
/// hibernate when the battery current is less than approximately C/100. While in hibernate mode,
/// the MAX1726x reduces its operating current to 5μA by reducing ADC sampling to once every
/// 5.625s. Table 24 shows the register format
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Default, Debug)]
pub struct HibCfg {
    /// HibScalar: Sets the task period while in hibernate mode based on the following equation:
    /// Hibernate Mode Task Period (s) = 351ms x 2^(HibScalar)
    hib_scalar: B3,

    /// HibExitTime: Sets the required time period of consecutive current readings above the
    /// HibThreshold value before the IC exits hibernate and returns to active mode of operation.
    /// Hibernate Mode Exit Time (s) = (HibExitTime + 1) x 702ms x 2^(HibScalar)
    hib_exit_time: B2,

    #[skip]
    __: B3,

    /// HibThreshold: Sets the threshold level for entering or exiting hibernate mode. The threshold is
    /// calculated as a fraction of the full capacity of the cell using the following equation:
    /// Hibernate Mode Threshold (mA) = (Full Cap (mAh)/0.8hrs)/(2^(HibThreshold))
    hib_threshold: B4,

    /// HibEnterTime: Sets the time period that consecutive current readings must remain below the
    /// HibThreshold value before the IC enters hibernate mode, as defined by the following equation.
    /// The default HibEnterTime value of 000b causes the IC to enter hibernate mode if all current
    /// readings are below the HibThreshold for a period of 5.625 seconds, but the IC could enter
    /// hibernate mode as quickly as 2.812 seconds.
    /// 2.812s x 2^(HibEnterTime) < Hibernate Mode Entry Time x 2.812s x 2^(HibEnterTime+1)
    hib_enter_time: B3,

    /// EnHib: Enable Hibernate Mode. When set to 1, the IC will enter hibernate mode if conditions
    /// are met. When set to 0, the IC always remains in the active mode of operation.
    en_hib: bool,
}

impl BitField for HibCfg {
    const REGISTER: u8 = Register::HIB_CFG;
}

impl HibCfg {
    /// Hibernate Mode Task Period (s)
    pub fn calc_hibernate_mode_task_period_s(&self) -> u32 {
        let base: u32 = 2;
        base.pow(self.hib_scalar() as u32) * 351
    }
    /// Hibernate Mode Exit Time (s)
    /// the required time period of consecutive current readings above the
    /// HibThreshold value before the IC exits hibernate and returns to active mode of operation
    pub fn calc_hibernate_mode_exit_time_s(&self) -> u32 {
        let base: u32 = 2;
        (self.hib_exit_time() as u32 + 1) * 702 * base.pow(self.hib_scalar() as u32)
    }

    /// Hibernate Mode Threshold (mA)
    /// Threshold level for entering or exiting hibernate mode
    /// * `full_cap_mah` - The full capacity of the cell in mAh
    pub fn calc_hibernate_mode_threshold_ma(&self, full_cap_mah: u32) -> f32 {
        let base: u32 = 2;
        (full_cap_mah as f32 / 0.8) / base.pow(self.hib_threshold() as u32) as f32
    }
}
impl defmt::Format for HibCfg {
    fn format(&self, f: defmt::Formatter) {
        // format the bitfields of the register
        defmt::write!(
            f,
            "Hibernate mode register:, \
                Bitfields: hib_scalar:{}, hib_exit_time:{}, hib_threshold:{}, hib_enter_time:{}, en_hib:{}, \
                Calculated: Task Period (s): {}, Exit time (s): {}", 
            self.hib_scalar(),
            self.hib_exit_time(),
            self.hib_threshold(),
            self.hib_enter_time(),
            self.en_hib(),
            self.calc_hibernate_mode_task_period_s(),
            self.calc_hibernate_mode_exit_time_s()
        )
    }
}

/// ModelCfg Register (DBh) (page 29)
/// Register Type: Special
/// Initial value: 0x8400
/// The ModelCFG register controls basic options of the EZ algorithm. Table 12 shows the register
/// format.
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Default, Debug)]
pub struct ModelCfg {
    #[skip]
    __: B2,
    #[skip]
    __: B2,
    /// ModelID: Choose from one of the following lithium models. For most batteries, use ModelID = 0.
    /// • ModelID = 0: Use for most lithium cobalt oxide variants (a large majority of lithium in the
    /// marketplace). Supported by EZ without characterization.
    /// • ModelID = 2: Use for lithium NCR or NCA cells such as Panasonic®
    /// . Supported by EZ
    /// without characterization.
    /// • ModelID = 6: Use for lithium iron phosphate (LiFePO4). For better performance, a
    /// custom characterization is recommended in this case, instead of an EZ configuration
    pub model_id: B4,

    #[skip]
    __: B2,
    /// VChg: Set VChg to 1 for a charge voltage higher than 4.25V (4.3V–4.4V). Set VChg to 0 for a
    ///4.2V charge voltage.
    pub v_chg: bool,

    #[skip]
    __: B2,
    /// R100: If using 100kΩ NTC, set R100 = 1; if using 10kΩ NTC, set R100 = 0.
    pub r100: bool,

    #[skip]
    __: B1,

    /// Refresh: Set Refresh to 1 to command the model reload. After execution, the MAX1726x clears
    ///Refresh to 0
    pub refresh: bool,
}

impl BitField for ModelCfg {
    const REGISTER: u8 = Register::MODEL_CFG;
}

/// VEmpty Register (3Ah) (page 28)
/// Initial Value: 0xA561 (3.3V / 3.88V)
/// The VEmpty register sets thresholds related to empty detection during operation. Table 11
/// shows the register format.
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Default, Debug)]
pub struct VEmpty {
    /// VE: Empty Voltage Target, during load. The fuel gauge provides capacity and percentage
    /// relative to the empty voltage target, eventually declaring 0% at VE. A 10mV resolution gives a
    /// range of 0 to 5.11V. This value is written to 3.3V after reset.
    pub ve: B9,
    /// VR: Recovery Voltage. Sets the voltage level for clearing empty detection. Once the cell voltage
    /// rises above this point, empty voltage detection is re-enabled. A 40mV resolution gives a range
    /// or 0 to 5.08V. This value is written to 3.88V, which is recommended for most applications.
    pub vr: B7,
}

impl BitField for VEmpty {
    const REGISTER: u8 = Register::V_EMPTY;
}

impl VEmpty {
    /// Get the empty voltage (in mV) target during load
    pub fn calc_empty_voltage_target_mv(&self) -> u16 {
        self.ve() * 10
    }
    /// Get the voltage (in mV) level for clearing empty detection
    pub fn calc_recovery_voltage_mv(&self) -> u16 {
        self.vr() as u16 * 40
    }

    // Create a new register with the provided values
    // * `empty_voltage_target_mv` - The empty voltage target during load, in mV
    // * `recovery_voltage_mv` - The voltage level for clearing empty detection, in mV
    pub fn init(empty_voltage_target_mv: u16, recovery_voltage_mv: u16) -> Self {
        Self::new()
            .with_ve(empty_voltage_target_mv / 10)
            .with_vr((recovery_voltage_mv / 40) as u8)
    }
}
impl defmt::Format for VEmpty {
    fn format(&self, f: defmt::Formatter) {
        // format the bitfields of the register
        defmt::write!(
            f,
            "Bitfields: ve:{}, vr:{}, \
            Calculated fields: Voltage empty (mV): {}, Recovery voltage (mV): {}",
            self.ve(),
            self.vr(),
            self.calc_empty_voltage_target_mv(),
            self.calc_recovery_voltage_mv()
        )
    }
}

/// Soft-Wakeup (Command Register 60h) (page 42)
/// Register Type: Special
/// To wake and exit hibernate:
/// 1. Write HibCfg = 0x0000.
/// 2. Soft-Wakeup Command. Write Command Register (60h) to 0x0090.
/// 3. Clear Command. Write Command Register (60h) to 0x0000.
/// 4. Eventually restore HibCfg to again allow automatic hibernate decisions.
pub struct SoftWakeup {}
impl SoftWakeup {
    /// Clears all commands.
    pub const CLEAR: u16 = 0;
    /// Wakes up the fuel gauge from hibernate mode to reduce the response time of the IC to
    /// configuration changes. This command must be manually cleared (0000h) afterward to keep proper
    /// fuel gauge timing.
    pub const SOFT_WAKEUP: u16 = 0x0090;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hib_cfg_bits() {
        // Test some hand-crafted bits (see the datasheet for more details)
        // These data are in MSB first/big-endian
        let mut bytes = [0b1010_1010u8, 0b0001_0111];
        bytes.reverse();
        // Needs to be LSB first/little-endian
        let mut hib_cfg = HibCfg::from_bytes(bytes);

        assert!(hib_cfg.en_hib());
        assert_eq!(hib_cfg.hib_enter_time(), 0b010);
        assert_eq!(hib_cfg.hib_threshold(), 0b1010);
        assert_eq!(hib_cfg.hib_exit_time(), 0b10);
        assert_eq!(hib_cfg.hib_scalar(), 0b111);

        hib_cfg.set_hib_threshold(4);
        assert_eq!(hib_cfg.hib_threshold(), 4);
    }
    #[test]
    fn model_cfg_bits() {
        // An example from the datasheet for when charge voltage > 4.275V
        let model_cfg = ModelCfg::from(0x8400);
        assert!(model_cfg.v_chg());
        assert_eq!(model_cfg.model_id(), 0);
        assert!(!model_cfg.r100());

        // An example from the datasheet for when charge voltage < 4.275V
        let model_cfg = ModelCfg::from(0x8000);
        assert!(!model_cfg.v_chg());
        assert_eq!(model_cfg.model_id(), 0);
        assert!(!model_cfg.r100());
    }
}
