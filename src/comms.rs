/// Refer to datasheet:
/// https://www.analog.com/media/en/technical-documentation/data-sheets/MAX17263.pdf
/// userguide:
/// https://www.analog.com/media/en/technical-documentation/user-guides/max1726x-modelgauge-m5-ez-user-guide.pdf
/// and software implementation guide:
/// https://www.analog.com/media/en/technical-documentation/user-guides/modelgauge-m5-host-side-software-implementation-guide.pdf
use crate::{
    error::Error,
    registers::{FStat, HibCfg, ModelCfg, OutputRegister, Register, SoftWakeup, Status, VEmpty},
    traits::{BitField, Model, RegisterResolver},
};
use core::fmt::Debug;
use embedded_hal::delay::DelayNs;
use embedded_hal::i2c;

#[derive(Debug, defmt::Format)]
pub struct Max1726x<'a, M, I2C, R>
where
    R: RegisterResolver,
{
    i2c: &'a mut I2C,
    register_resolver: R,
    _phantom: core::marker::PhantomData<M>,
}

// The MAX1726x supports the slave address 0x6C
// The datasheet specifies an I2C slave address of 0x6C, i.e. 01101100
// For the HAL, you need to remove the LSB, which turns it into 0110110 or 0x36
const ADDR: u8 = 0x36;

/// EzConfig struct - see step 2.1 (page 7) of ModelGauge m5 Host Side Software
/// Implementation Guide UG6595; Rev 4; 12/21
#[derive(Debug, defmt::Format)]
pub struct EzConfig {
    /// * `charge_voltage`: in millivolts
    pub charge_voltage_mv: u16,
    /// * `design_cap`: the expected capacity of the cell in mAh
    pub design_cap_mah: u16,
    /// * `i_chg_term`: in milliamps
    pub i_chg_term_ma: u16,
    /// * `v_empty`: see VEmpty struct configuration
    pub v_empty_mv: VEmpty,
}

/// Battery charge status
#[derive(Debug, defmt::Format)]
pub struct BatteryChargeStatus {
    /// RepCap or reported remaining capacity in mAh.
    rep_cap: u16,

    /// RepSOC is the reported state-of-charge percentage output
    rep_soc: u16,

    /// TTE is the estimated time to empty for the application under present
    /// temperature and load conditions. The TTE value is determined by relating AvCap with
    /// The LSB of the TTE register is 5.625s.
    tte: u16,
}

impl<'a, M, I2C, E, R> Max1726x<'a, M, I2C, R>
where
    M: Model,
    I2C: i2c::I2c<Error = E>,
    E: i2c::Error,
    R: RegisterResolver,
{
    /// Create a new driver instance.
    pub fn new(i2c: &'a mut I2C, register_resolver: R) -> Self {
        Self {
            i2c,
            register_resolver,
            _phantom: core::marker::PhantomData,
        }
    }

    /// Write a register - data should be written little endian/LSB first
    pub fn write_register(&mut self, register: u8, data: u16) -> Result<(), Error<E>> {
        let payload: [u8; 3] = [register, (data & 0xFF) as u8, ((data & 0xFF00) >> 8) as u8];
        self.i2c.write(ADDR, &payload).map_err(Error::I2c)
    }

    pub fn write_and_verify_register<D>(
        &mut self,
        register: u8,
        data: u16,
        mut delay: D,
    ) -> Result<(), Error<E>>
    where
        D: DelayNs,
    {
        let mut attempt: u8 = 0;
        loop {
            self.write_register(register, data)?;
            delay.delay_ms(1);
            let read_value = self.read_register_as_u16(register)?;
            if data == read_value {
                return Ok(());
            }
            attempt += 1;
            if attempt > 3 {
                return Err(Error::WriteNotVerified {
                    register,
                    write: data,
                    read: read_value,
                });
            }
        }
    }

    pub fn write_bitfield_to_register<B>(&mut self, bitfield: B) -> Result<(), Error<E>>
    where
        B: Into<u16> + BitField,
    {
        let data = bitfield.into();
        self.write_register(B::REGISTER, data)
    }

    /// Read a register - return the bytes in the order that they are received (litte-endian/LSB
    /// first)
    pub fn read_register(&mut self, register: u8) -> Result<[u8; 2], Error<E>> {
        let mut data = [0u8; 2];
        self.i2c
            .write_read(ADDR, &[register], &mut data)
            .map_err(Error::I2c)
            .and(Ok(data))
    }

    /// Read a register into a u16
    pub fn read_register_as_u16(&mut self, register: u8) -> Result<u16, Error<E>> {
        let data = self.read_register(register)?;
        Ok((u16::from(data[1]) << 8) | u16::from(data[0]))
    }

    /// Read a register and convert to the given bitfield
    pub fn read_register_as_bitfield<B>(&mut self) -> Result<B, Error<E>>
    where
        B: From<u16> + BitField,
    {
        Ok(B::from(self.read_register_as_u16(B::REGISTER)?))
    }

    /// Battery voltage in V
    pub fn battery_voltage(&mut self) -> Result<f64, Error<E>> {
        let register = self.read_register_as_u16(M::V_CELL)?;
        Ok(self.register_resolver.register_to_voltage(register))
    }

    /// Battery charge/discharge current in A
    pub fn battery_current(&mut self) -> Result<f64, Error<E>> {
        let register = self.read_register_as_u16(M::CURRENT)?;
        Ok(self.register_resolver.register_to_current(register))
    }

    /// Battery temperature in degrees C
    pub fn battery_temperature(&mut self) -> Result<f64, Error<E>> {
        let register = self.read_register_as_u16(M::TEMP)?;
        Ok(self.register_resolver.register_to_temperature(register))
    }

    /// Battery state of charge as a percentage
    pub fn battery_state_of_charge(&mut self) -> Result<f64, Error<E>> {
        let register = self.read_register_as_u16(OutputRegister::REP_SOC)?;
        Ok(self.register_resolver.register_to_percentage(register))
    }

    /// Get Status register (00h)
    pub fn status_register(&mut self) -> Result<Status, Error<E>> {
        Ok(Status::from_bits_truncate(
            self.read_register_as_u16(Register::STATUS)?,
        ))
    }
    /// Get FStat Register (3Dh)
    pub fn fstat_register(&mut self) -> Result<FStat, Error<E>> {
        Ok(FStat::from_bits_truncate(
            self.read_register_as_u16(Register::F_STAT)?,
        ))
    }
    /// Get HibCfg Register (BAh)
    pub fn hib_cfg_register(&mut self) -> Result<HibCfg, Error<E>> {
        let msb_bytes = self.read_register_as_u16(Register::HIB_CFG)?;
        Ok(HibCfg::from_bytes(msb_bytes.to_le_bytes()))
    }

    /// Get VEmpty Register (3Ah)
    pub fn v_empty_register(&mut self) -> Result<VEmpty, Error<E>> {
        let msb_bytes = self.read_register_as_u16(Register::V_EMPTY)?;
        Ok(VEmpty::from_bytes(msb_bytes.to_le_bytes()))
    }

    /// Get the battery charge status
    pub fn battery_charge_status(&mut self) -> Result<BatteryChargeStatus, Error<E>> {
        let rep_cap = self.read_register_as_u16(OutputRegister::REP_CAP)?;
        let rep_soc = self.read_register_as_u16(OutputRegister::REP_SOC)?;
        let tte = self.read_register_as_u16(OutputRegister::TTE)?;
        Ok(BatteryChargeStatus {
            rep_cap,
            rep_soc,
            tte,
        })
    }

    /// Setup the fuel gauge as per:
    /// ModelGauge m5 Host Side Software Implementation Guide UG6595; Rev 4; 12/21
    /// https://www.analog.com/media/en/technical-documentation/user-guides/modelgauge-m5-host-side-software-implementation-guide.pdf
    /// page 6
    pub fn ez_config<D>(&mut self, mut delay: D, ez_config: EzConfig) -> Result<(), Error<E>>
    where
        D: DelayNs,
    {
        defmt::info!(
            "Starting MAX1726x EZ Config with configuration: {}",
            ez_config
        );
        // Step 0: check for POR
        defmt::info!("Checking for Power On Reset (POR)");
        let status = self.status_register()?;
        defmt::info!("Initial status: {}", status);
        let status_por = status & Status::POR;
        if status_por.is_empty() {
            defmt::info!("Power On Reset (POR) not detected.");
            // Go to step 3.2
        } else {
            // Do steps 1-2

            // Step 1. Delay until FSTAT.DNR bit == 0
            defmt::info!("Delaying for DNR bit to clear...");
            while !(self.fstat_register()? & FStat::DNR).is_empty() {
                // 10ms Wait Loop. Do not continue until FSTAT.DNR==0
                delay.delay_ms(10);
            }

            defmt::info!("DNR bit cleared. Proceeding to Step 2.");
            // Step 2. Initialise configuration
            // Store original HibCFG value
            let hib_cfg = self.hib_cfg_register()?;
            defmt::info!("HibCFG: {}", hib_cfg);
            defmt::info!("Exiting Hibernate Mode");
            // Exit Hibernate Mode step 1
            self.write_register(Register::SOFT_WAKEUP, SoftWakeup::SOFT_WAKEUP)?;
            // Exit Hibernate Mode step 2
            self.write_register(Register::HIB_CFG, 0)?;
            // Exit Hibernate Mode step 3
            self.write_register(Register::SOFT_WAKEUP, SoftWakeup::CLEAR)?;

            // 2.1 OPTION 1 EZ Config (No INI file is needed):
            defmt::info!("Option 1 EZ Config");
            defmt::info!("Writing DESIGN_CAP, I_CHG_TERM, V_EMPTY");
            self.write_register(Register::DESIGN_CAP, ez_config.design_cap_mah)?;
            self.write_register(Register::I_CHG_TERM, ez_config.i_chg_term_ma)?;
            self.write_register(
                Register::V_EMPTY,
                u16::from_le_bytes(ez_config.v_empty_mv.into_bytes()),
            )?;
            defmt::info!("Writing ModelCFG");
            if ez_config.charge_voltage_mv > 4275 {
                self.write_register(0xDB, 0x8400)?; // Write ModelCFG
            } else {
                self.write_register(0xDB, 0x8000)?; // Write ModelCFG
            }

            // Poll ModelCFG.Refresh(highest bit),
            // proceed to Step 3 when ModelCFG.Refresh=0.
            defmt::info!("Waiting for ModelCFG.Refresh to clear...");
            while self.read_register_as_bitfield::<ModelCfg>()?.refresh() {
                delay.delay_ms(10);
            }
            // do not continue until ModelCFG.Refresh==0

            // Restore Original HibCFG value
            defmt::info!("Restoring Hibernate Mode");
            self.write_bitfield_to_register(hib_cfg)?;

            // Proceed to Step 3.
        }
        defmt::info!("Proceeding to Step 3.");
        // Step 3: Initialization Complete
        // Clear the POR bit to indicate that the custom model and parameters are successfully loaded.
        // Read Status
        defmt::info!("Clearing Power On Reset (POR) bit");
        let status = self.read_register_as_u16(Register::STATUS)?;
        self.write_register(Register::STATUS, status & 0xFFFD)?;
        // Write and Verify Status with POR bit Cleared
        defmt::info!("Verifying status with cleared POR bit");
        // Write and Verify Status with POR bit Cleared
        self.write_and_verify_register(Register::STATUS, status & 0xFFFD, delay)?;
        defmt::info!("Initialization Complete");

        defmt::info!("Battery charge status:");
        let battery_charge_status = self.battery_charge_status()?;
        defmt::info!("{}", battery_charge_status);

        Ok(())
    }
}
