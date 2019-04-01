use rppal::i2c::I2c;
use ssd1306::{mode::GraphicsMode, Builder, prelude::*};
use embedded_graphics::fonts::Font6x8;
use embedded_graphics::primitives::circle::Circle;
use embedded_graphics::prelude::*;

fn main() {
    let i2c = I2c::new().unwrap();
    let mut disp: GraphicsMode<_> = Builder::new()
        .with_size(DisplaySize::Display128x32)
        .connect_i2c(i2c)
        .into();

    disp.init().unwrap();
    disp.flush().unwrap();
    disp.draw(
        Font6x8::render_str("Hello world!")
        .with_style(Style::with_stroke(1u8.into()))
        .into_iter()
    );
    disp.draw(
        Circle::new(Coord::new(64, 15), 5)
        .with_style(Style::with_stroke(1u8.into()))
        .into_iter()
    );
    disp.flush().unwrap();
}
