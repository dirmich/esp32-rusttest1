use esp_idf_hal::{
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};

use crate::config;

const _: () = assert!(config::SDA_PIN == 5);
const _: () = assert!(config::SCL_PIN == 6);

pub fn create_i2c_driver(peripherals: Peripherals) -> anyhow::Result<I2cDriver<'static>> {
    let i2c_config = I2cConfig::new().baudrate(config::I2C_BAUDRATE_HZ.Hz());

    // This ESP32-C3 0.42" OLED board routes the onboard display to SDA=GPIO5, SCL=GPIO6.
    // When changing pins, keep config.rs and this mapping in sync.
    let i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio5,
        peripherals.pins.gpio6,
        &i2c_config,
    )?;

    Ok(i2c)
}
