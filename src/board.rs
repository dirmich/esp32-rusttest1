#[cfg(feature = "board-heltec")]
use esp_idf_hal::{delay::FreeRtos, gpio::PinDriver};
use esp_idf_hal::{
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};

use crate::config;

pub fn create_i2c_driver(peripherals: Peripherals) -> anyhow::Result<I2cDriver<'static>> {
    let i2c_config = I2cConfig::new().baudrate(config::I2C_BAUDRATE_HZ.Hz());

    log_selected_board();

    #[cfg(feature = "board-original")]
    let i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio5,
        peripherals.pins.gpio6,
        &i2c_config,
    )?;

    #[cfg(feature = "board-heltec")]
    reset_heltec_oled(peripherals.pins.gpio16)?;

    #[cfg(feature = "board-heltec")]
    let i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio4,
        peripherals.pins.gpio15,
        &i2c_config,
    )?;

    #[cfg(feature = "board-esp32-cam")]
    let i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio14,
        peripherals.pins.gpio15,
        &i2c_config,
    )?;

    Ok(i2c)
}

fn log_selected_board() {
    match config::OLED_RESET_PIN {
        Some(reset_pin) => println!(
            "board: {}, oled=0x{:02x}, sda=GPIO{}, scl=GPIO{}, reset=GPIO{}, camera={}, lora={}",
            config::BOARD_NAME,
            config::OLED_I2C_ADDRESS,
            config::SDA_PIN,
            config::SCL_PIN,
            reset_pin,
            config::HAS_CAMERA,
            config::HAS_LORA,
        ),
        None => println!(
            "board: {}, oled=0x{:02x}, sda=GPIO{}, scl=GPIO{}, reset=none, camera={}, lora={}",
            config::BOARD_NAME,
            config::OLED_I2C_ADDRESS,
            config::SDA_PIN,
            config::SCL_PIN,
            config::HAS_CAMERA,
            config::HAS_LORA,
        ),
    }
}

#[cfg(feature = "board-heltec")]
fn reset_heltec_oled(reset_pin: esp_idf_hal::gpio::Gpio16) -> anyhow::Result<()> {
    let mut reset = PinDriver::output(reset_pin)?;
    reset.set_low()?;
    FreeRtos::delay_ms(10);
    reset.set_high()?;
    FreeRtos::delay_ms(10);
    Ok(())
}
