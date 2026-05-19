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
    prepare_heltec_oled(peripherals.pins.gpio21, peripherals.pins.gpio16)?;

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
    println!(
        "board: {}, oled=0x{:02x}, sda=GPIO{}, scl=GPIO{}, reset={}, power={}, rotation={}deg, camera={}, lora={}",
        config::BOARD_NAME,
        config::OLED_I2C_ADDRESS,
        config::SDA_PIN,
        config::SCL_PIN,
        pin_label(config::OLED_RESET_PIN),
        pin_label(config::OLED_POWER_PIN),
        config::OLED_ROTATION_DEGREES,
        config::HAS_CAMERA,
        config::HAS_LORA,
    );
}

fn pin_label(pin: Option<i32>) -> String {
    match pin {
        Some(pin) => format!("GPIO{pin}"),
        None => "none".to_owned(),
    }
}

#[cfg(feature = "board-heltec")]
fn prepare_heltec_oled(
    power_pin: esp_idf_hal::gpio::Gpio21,
    reset_pin: esp_idf_hal::gpio::Gpio16,
) -> anyhow::Result<()> {
    let mut power = PinDriver::output(power_pin)?;
    power.set_low()?;
    FreeRtos::delay_ms(50);

    let mut reset = PinDriver::output(reset_pin)?;
    reset.set_low()?;
    FreeRtos::delay_ms(10);
    reset.set_high()?;
    FreeRtos::delay_ms(50);

    // Keep GPIO21 low and GPIO16 high for the life of the firmware. Dropping the
    // drivers releases the pins and can depower or reset the Heltec OLED.
    core::mem::forget(power);
    core::mem::forget(reset);

    Ok(())
}
