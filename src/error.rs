use core::fmt::Debug;
use embedded_hal::i2c;

/// The error type used by this library.
///
/// This can encapsulate an I2C error, and adds its own protocol errors
/// on top of that.
#[derive(defmt::Format)]
pub enum Error<E: i2c::Error>
where
    E: Debug + 'static + Sized,
{
    /// An I2C error occurred, or the I2C transaction failed.
    I2c(E),

    /// Written data not verified when read back
    /// * `register`: register address
    /// * `write`: register value written
    /// * `read`: register value read
    WriteNotVerified { register: u8, write: u16, read: u16 },
}

impl<E> Debug for Error<E>
where
    E: i2c::Error,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use Error::*;
        match self {
            I2c(err) => write!(f, "I2C error: {:?}", err.kind()),
            WriteNotVerified{register, write, read} => write!(f, "Written data not verified to register {:x}. Regisiter value written: {:x}, read: {:x}", register,write, read)
        }
    }
}
