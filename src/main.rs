use rppal::i2c::I2c;
use ssd1306::{mode::TerminalMode, Builder};

fn main() {
    let i2c = I2c::new().unwrap();
    let mut disp: TerminalMode<_> = Builder::new().connect_i2c(i2c).into();
    disp.init().unwrap();
    disp.clear().unwrap();
    disp.print_char('a').unwrap();
}
