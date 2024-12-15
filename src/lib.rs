#![cfg_attr(not(test), no_std)]

use embedded_hal_async::i2c::I2c;

#[macro_use]
extern crate bitflags;

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
    value: [u8; 1],
}

/// Enum representing the I²C registers of the ADP5360.
pub enum Register {
    /// Manufacturer and Model ID.
    ManufacturerModelId = 0x00,
    /// Silicon Revision.
    SiliconRevision = 0x01,
    /// Charger VBUS ILIM.
    ChargerVbusIlim = 0x02,
    /// Charger Termination Setting.
    ChargerTerminationSetting = 0x03,
    /// Charger Current Setting.
    ChargerCurrentSetting = 0x04,
    /// Charger Voltage Threshold.
    ChargerVoltageThreshold = 0x05,
    /// Charger Timer Setting.
    ChargerTimerSetting = 0x06,
    /// Charger Function Setting.
    ChargerFunctionSetting = 0x07,
    /// Charger Status 1.
    ChargerStatus1 = 0x08,
    /// Charger Status 2.
    ChargerStatus2 = 0x09,
    /// Battery Thermistor Control.
    BatteryThermistorControl = 0x0A,
    /// Thermistor 60°C Threshold.
    Thermistor60CThreshold = 0x0B,
    /// Thermistor 45°C Threshold.
    Thermistor45CThreshold = 0x0C,
    /// Thermistor 10°C Threshold.
    Thermistor10CThreshold = 0x0D,
    /// Thermistor 0°C Threshold.
    Thermistor0CThreshold = 0x0E,
    /// Threshold Voltage Low.
    ThresholdVoltageLow = 0x0F,
    /// Threshold Voltage High.
    ThresholdVoltageHigh = 0x10,
    /// Battery Protection Control.
    BatteryProtectionControl = 0x11,
    /// Battery Protection Undervoltage Setting.
    BatteryProtectionUndervoltageSetting = 0x12,
    /// Battery Protection Overcharge Setting.
    BatteryProtectionOverchargeSetting = 0x13,
    /// Battery Protection Overvoltage Setting.
    BatteryProtectionOvervoltageSetting = 0x14,
    /// Battery Protection Charge Overcharge Setting.
    BatteryProtectionChargeOverchargeSetting = 0x15,
    /// Voltage SOC 0.
    VoltageSoc0 = 0x16,
    /// Voltage SOC 5.
    VoltageSoc5 = 0x17,
    /// Voltage SOC 11.
    VoltageSoc11 = 0x18,
    /// Voltage SOC 19.
    VoltageSoc19 = 0x19,
    /// Voltage SOC 28.
    VoltageSoc28 = 0x1A,
    /// Voltage SOC 41.
    VoltageSoc41 = 0x1B,
    /// Voltage SOC 55.
    VoltageSoc55 = 0x1C,
    /// Voltage SOC 69.
    VoltageSoc69 = 0x1D,
    /// Voltage SOC 84.
    VoltageSoc84 = 0x1E,
    /// Voltage SOC 100.
    VoltageSoc100 = 0x1F,
    /// Battery Capacity.
    BatteryCapacity = 0x20,
    /// Battery SOC.
    BatterySoc = 0x21,
    /// Battery SOC Accumulation Control.
    BatterySocAccumulationControl = 0x22,
    /// Battery SOC Accumulation High.
    BatterySocAccumulationHigh = 0x23,
    /// Battery SOC Accumulation Low.
    BatterySocAccumulationLow = 0x24,
    /// PGOOD Status.
    PGoodStatus = 0x2F,
    /// PGOOD1 Mask.
    PGood1Mask = 0x30,
    /// PGOOD2 Mask.
    PGood2Mask = 0x31,
    /// Interrupt Enable 1.
    InterruptEnable1 = 0x32,
    /// Interrupt Enable 2.
    InterruptEnable2 = 0x33,
    /// Interrupt Flag 1.
    InterruptFlag1 = 0x34,
    /// Interrupt Flag 2.
    InterruptFlag2 = 0x35,
    /// Ship Mode.
    ShipMode = 0x36,
}

bitflags! {
    /// Bitfield definitions for Charger Function Setting Register (0x07)
    pub struct ChargerFunctionSetting: u8 {
        const EN_CHG = 1 << 0;           // Enable Charging
        const EN_ADPICHG = 1 << 1;       // Enable Adaptive Charging
        const EN_EOC = 1 << 2;           // Enable End of Charge (EOC)
        const EN_LDO = 1 << 3;           // Enable LDO
        const OFF_ISOFET = 1 << 4;       // Turn Off the ISOFET
        const RESERVED = 1 << 5;         // Reserved Bit
        const ILIM_JEITA_COOL = 1 << 6;  // JEITA Cool Current Limit
        const EN_JEITA = 1 << 7;         // Enable JEITA Temperature Profile
    }
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
            value: [0],
        }
    }

    /// Writes a byte value to a specified register.
    ///
    /// # Arguments
    ///
    /// * `register` - The register to write to
    /// * `value` - The byte value to write
    ///
    /// # Returns
    ///
    /// A Result indicating success or an I2C bus error
    async fn write_register(&mut self, register: Register, value: u8) -> Result<(), I2C::Error> {
        self.i2c.write(self.address, &[register as u8, value]).await
    }

    /// Reads a byte value from a specified register.
    ///
    /// # Arguments
    ///
    /// * `register` - The register to read from
    ///
    /// # Returns
    ///
    /// A Result containing the byte value read or an I2C bus error
    async fn read_register(&mut self, register: Register) -> Result<u8, I2C::Error> {
        let result = self
            .i2c
            .write_read(self.address, &[register as u8], &mut self.value)
            .await;
        match result {
            Ok(()) => Ok(self.value[0]),
            Err(e) => Err(e),
        }
    }

    /// Enables the battery charger.
    ///
    /// This function sets the enable bit in the charger control register.
    ///
    /// # Returns
    ///
    /// A Result indicating success or an I2C bus error
    pub async fn enable_charger(&mut self) -> Result<(), I2C::Error> {
        self.write_register(
            Register::ChargerFunctionSetting,
            ChargerFunctionSetting::EN_CHG.bits(),
        )
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
    pub async fn read_battery_voltage(&mut self) -> Result<u8, I2C::Error> {
        self.read_register(Register::BatterySoc).await
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
    async fn test_read_battery_voltage() {
        let expectations = [
            I2cTransaction::write_read(0x68, vec![Register::BatterySoc as u8], vec![0x12]), // Read battery voltage register
        ];
        let mut i2c = I2cMock::new(&expectations);

        let mut adp5360 = ADP5360::new(i2c.clone(), 0x68);
        let result = adp5360.read_battery_voltage().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x12); // Check the combined bytes

        i2c.done();
    }
    #[tokio::test]
    async fn test_read_register() {
        let expectations = [
            I2cTransaction::write_read(0x68, vec![Register::ChargerStatus1 as u8], vec![0x55]), // Read from arbitrary register
        ];
        let mut i2c = I2cMock::new(&expectations);

        let mut adp5360 = ADP5360::new(i2c.clone(), 0x68);
        let result = adp5360.read_register(Register::ChargerStatus1).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x55);

        i2c.done();
    }

    #[tokio::test]
    async fn test_write_register() {
        let expectations = [I2cTransaction::write(
            0x68,
            vec![Register::ChargerFunctionSetting as u8, 0x01],
        )];
        let mut i2c = I2cMock::new(&expectations);

        let mut adp5360 = ADP5360::new(i2c.clone(), 0x68);
        let result = adp5360
            .write_register(Register::ChargerFunctionSetting, 0x01)
            .await;
        assert!(result.is_ok());

        i2c.done();
    }
}
