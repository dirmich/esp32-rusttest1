use esp_idf_hal::{delay::FreeRtos, i2c::I2cDriver};

use crate::{config, display::OledDisplay};

pub fn run(i2c: I2cDriver<'static>) -> anyhow::Result<()> {
    let mut oled = OledDisplay::new(i2c, config::OLED_I2C_ADDRESS)?;
    oled.show_text(config::DISPLAY_TEXT)?;

    loop {
        FreeRtos::delay_ms(1_000);
    }
}
