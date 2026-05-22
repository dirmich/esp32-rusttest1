use esp_idf_hal::{delay::FreeRtos, i2c::I2cDriver};

use crate::{config, display::OledDisplay};

pub fn run(i2c: I2cDriver<'static>) -> anyhow::Result<()> {
    let mut oled = OledDisplay::new(i2c, config::OLED_I2C_ADDRESS)?;
    oled.show_text(config::DISPLAY_TEXT)?;

    keep_alive();
}

pub fn run_without_oled() -> ! {
    println!("OLED display is not configured for this board; keeping firmware alive.");

    loop {
        unsafe {
            esp_idf_sys::vTaskDelay(1_000 / esp_idf_sys::portTICK_PERIOD_MS);
        }
    }
}

fn keep_alive() -> ! {
    loop {
        FreeRtos::delay_ms(1_000);
    }
}
