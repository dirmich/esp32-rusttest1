use esp_idf_hal::peripherals::Peripherals;

mod app;
mod board;
mod config;
mod display;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take()?;
    let i2c = board::create_i2c_driver(peripherals)?;
    app::run(i2c)
}
