use adp5360::ADP5360;
// Replace this with the actual I2C implementation of your async HAL
use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction};

#[tokio::main]
async fn main() {
    // Create mock I2C device
    let expectations = [
        Transaction::write(0x68, vec![0x07, 0x01]), // Enable charger
        Transaction::write_read(0x68, vec![0x10], vec![0x12, 0x34]), // Read battery voltage
    ];
    let i2c = I2cMock::new(&expectations);

    // Create ADP5360 instance
    let mut adp5360 = ADP5360::new(i2c, 0x68);

    // Enable charger
    adp5360.enable_charger().await.unwrap();

    // Read battery voltage
    let voltage = adp5360.read_battery_voltage().await.unwrap();
    println!("Battery voltage: {:#04x}", voltage);
}
