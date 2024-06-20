use core::fmt::Debug;
use embedded_hal::i2c;

/// The error type used by this library.
///
/// This can encapsulate an I2C error, and adds its own protocol errors
/// on top of that.
#[derive(Debug, defmt::Format)]
pub enum Error<E: i2c::Error>
where
    E: Debug,
{
    /// An I2C error occurred, or the I2C transaction failed.
    I2c(E),
}
