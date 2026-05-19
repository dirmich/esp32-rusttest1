use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Baseline, Text},
};
use embedded_hal::i2c::I2c;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

use crate::config;

type DisplaySize = DisplaySize128x64;

pub struct OledDisplay<I2C> {
    display: Ssd1306<I2CInterface<I2C>, DisplaySize, BufferedGraphicsMode<DisplaySize>>,
}

impl<I2C, E> OledDisplay<I2C>
where
    I2C: I2c<Error = E>,
    E: core::fmt::Debug,
{
    pub fn new(i2c: I2C, address: u8) -> anyhow::Result<Self> {
        let interface = I2CDisplayInterface::new_custom_address(i2c, address);
        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();

        display
            .init()
            .map_err(|err| anyhow::anyhow!("OLED init failed: {err:?}"))?;

        Ok(Self { display })
    }

    pub fn show_text(&mut self, text: &str) -> anyhow::Result<()> {
        self.display
            .clear(BinaryColor::Off)
            .map_err(|err| anyhow::anyhow!("OLED clear failed: {err:?}"))?;

        let viewport_origin = Point::new(config::VIEWPORT_X_OFFSET, config::VIEWPORT_Y_OFFSET);
        let viewport_size = Size::new(
            config::VIEWPORT_WIDTH as u32,
            config::VIEWPORT_HEIGHT as u32,
        );

        Rectangle::new(viewport_origin, viewport_size)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut self.display)
            .map_err(|err| anyhow::anyhow!("OLED frame draw failed: {err:?}"))?;

        let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let text_width = text.chars().count() as i32 * FONT_6X10.character_size.width as i32;
        let text_origin = viewport_origin
            + Point::new(
                (config::VIEWPORT_WIDTH - text_width) / 2,
                config::VIEWPORT_HEIGHT / 2,
            );

        Text::with_baseline(text, text_origin, style, Baseline::Middle)
            .draw(&mut self.display)
            .map_err(|err| anyhow::anyhow!("OLED draw failed: {err:?}"))?;

        self.display
            .flush()
            .map_err(|err| anyhow::anyhow!("OLED flush failed: {err:?}"))?;

        Ok(())
    }
}
