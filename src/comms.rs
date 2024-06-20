/// Refer to datasheet:
/// https://www.analog.com/media/en/technical-documentation/data-sheets/MAX17263.pdf
/// userguide:
/// https://www.analog.com/media/en/technical-documentation/user-guides/max1726x-modelgauge-m5-ez-user-guide.pdf
/// and software implementation guide:
/// https://www.analog.com/media/en/technical-documentation/user-guides/modelgauge-m5-host-side-software-implementation-guide.pdf
use crate::error::Error;
use core::fmt::Debug;
use embedded_hal::i2c;

#[derive(Debug, defmt::Format)]
pub struct Max1726x<I2C> {
    i2c: I2C,
}

// The MAX1726x supports the slave address 0x6C
// The datasheet specifies an I2C slave address of 0x6C, i.e. 01101100
// For the HAL, you need to remove the LSB, which turns it into 0110110 or 0x36
const ADDR: u8 = 0x36;

pub struct Register;
impl Register {
    pub const STATUS: u8 = 0x00;
}

impl<I2C, E> Max1726x<I2C>
where
    I2C: i2c::I2c<Error = E>,
    E: i2c::Error,
{
    /// Create a new driver instance.
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    /// Destroy driver instance, return I2C bus.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Write a register
    pub(crate) fn write_register(&mut self, register: u8, data: u16) -> Result<(), Error<E>> {
        let payload: [u8; 3] = [register, ((data & 0xFF00) >> 8) as u8, (data & 0xFF) as u8];
        self.i2c.write(ADDR, &payload).map_err(Error::I2c)
    }

    /// Read a register
    pub(crate) fn read_register(&mut self, register: u8) -> Result<u16, Error<E>> {
        let mut data = [0; 2];
        self.i2c
            .write_read(ADDR, &[register], &mut data)
            .map_err(Error::I2c)
            .and(Ok((u16::from(data[0]) << 8) | u16::from(data[1])))
    }

    /// Get IC version
    pub fn status(&mut self) -> Result<u16, Error<E>> {
        self.read_register(Register::STATUS)
    }
}
