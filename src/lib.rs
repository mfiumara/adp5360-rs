#![cfg_attr(not(test), no_std)]

use core::future::Future;

use embedded_hal_async::i2c::I2c;

/// Driver for the ADP5360 Power Management IC.
///
/// This driver provides an interface to control and monitor the ADP5360 PMIC,
/// including battery charging control and voltage monitoring capabilities.
///
/// # Example
///
/// ```no_run
/// # use embedded_hal_async::i2c::I2c;
/// # async fn example<I2C: I2c>(i2c: I2C) {
/// use adp5360::ADP5360;
///
/// let mut pmic = ADP5360::new(i2c, 0x68);
///
/// // Enable battery charging
/// pmic.enable_charger().await.unwrap();
///
/// // Read battery voltage
/// let voltage = pmic.read_battery_voltage().await.unwrap();
/// # }
/// ```
pub struct ADP5360<I2C> {
    i2c: I2C,
    address: u8,
    data: [u8; 2],
}

impl<I2C> ADP5360<I2C>
where
    I2C: I2c,
{
    /// Creates a new ADP5360 driver.
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C bus implementation
    /// * `address` - The 7-bit I2C address of the device (typically 0x68)
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self {
            i2c,
            address,
            data: [0, 0],
        }
    }

    /// Writes a value to a register.
    ///
    /// # Arguments
    ///
    /// * `register` - The register address to write to
    /// * `value` - The value to write
    ///
    /// # Returns
    ///
    /// A Future that resolves to Result<(), Error> where Error is the I2C bus error type
    pub fn write_register(
        &mut self,
        register: u8,
        value: u8,
    ) -> impl Future<Output = Result<(), I2C::Error>> + '_ {
        self.data = [register, value];
        self.i2c.write(self.address, &self.data)
    }

    /// Reads a value from a register.
    ///
    /// # Arguments
    ///
    /// * `register` - The register address to read from
    ///
    /// # Returns
    ///
    /// A Result containing the read value or an I2C bus error
    pub async fn read_register(&mut self, register: u8) -> Result<u8, I2C::Error> {
        let mut buffer = [0];
        self.i2c
            .write_read(self.address, &[register], &mut buffer)
            .await?;
        Ok(buffer[0])
    }

    /// Enables the battery charger.
    ///
    /// This function sets the enable bit in the charger control register.
    ///
    /// # Returns
    ///
    /// A Result indicating success or an I2C bus error
    pub async fn enable_charger(&mut self) -> Result<(), I2C::Error> {
        const CHARGER_ENABLE_REGISTER: u8 = 0x07;
        const ENABLE_BIT: u8 = 0x01;
        self.write_register(CHARGER_ENABLE_REGISTER, ENABLE_BIT)
            .await
    }

    /// Reads the battery voltage.
    ///
    /// This function reads the battery voltage register which returns a 16-bit value
    /// representing the current battery voltage.
    ///
    /// # Returns
    ///
    /// A Result containing the battery voltage as a 16-bit value or an I2C bus error
    pub async fn read_battery_voltage(&mut self) -> Result<u16, I2C::Error> {
        const BATTERY_VOLTAGE_REGISTER: u8 = 0x10;
        let mut buffer = [0, 0];
        self.i2c
            .write_read(self.address, &[BATTERY_VOLTAGE_REGISTER], &mut buffer)
            .await?;
        Ok(u16::from_be_bytes(buffer))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

    #[tokio::test]
    async fn test_enable_charger() {
        let expectations = [
            I2cTransaction::write(0x68, vec![0x07, 0x01]), // Write enable command
        ];
        let mut i2c = I2cMock::new(&expectations);

        let mut adp5360 = ADP5360::new(i2c.clone(), 0x68);
        assert!(adp5360.enable_charger().await.is_ok());

        i2c.done(); // Verify all expectations were met
    }

    #[tokio::test]
    async fn test_write_register() {
        let expectations = [
            I2cTransaction::write(0x68, vec![0x42, 0x55]), // Write arbitrary register and value
        ];
        let mut i2c = I2cMock::new(&expectations);

        let mut adp5360 = ADP5360::new(i2c.clone(), 0x68);
        assert!(adp5360.write_register(0x42, 0x55).await.is_ok());

        i2c.done();
    }

    #[tokio::test]
    async fn test_read_register() {
        let expectations = [
            I2cTransaction::write_read(0x68, vec![0x42], vec![0x55]), // Read arbitrary register
        ];
        let mut i2c = I2cMock::new(&expectations);

        let mut adp5360 = ADP5360::new(i2c.clone(), 0x68);
        let result = adp5360.read_register(0x42).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x55);

        i2c.done();
    }

    #[tokio::test]
    async fn test_read_battery_voltage() {
        let expectations = [
            I2cTransaction::write_read(0x68, vec![0x10], vec![0x12, 0x34]), // Read battery voltage register
        ];
        let mut i2c = I2cMock::new(&expectations);

        let mut adp5360 = ADP5360::new(i2c.clone(), 0x68);
        let result = adp5360.read_battery_voltage().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x1234); // Check the combined bytes

        i2c.done();
    }
}
