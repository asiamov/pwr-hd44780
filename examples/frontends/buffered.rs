extern crate pwr_hd44780;

use pwr_hd44780::Hd44780;

fn main() {
    // create the LCD's interface
    let mut lcd_interface = pwr_hd44780::interface::I2C::new("/dev/i2c-1", 0x27);

    // create the LCD's frontend
    let mut direct_lcd = pwr_hd44780::frontend::Direct::new(
        &mut lcd_interface,

        pwr_hd44780::Properties {
            width: 20,
            height: 4,
            font: pwr_hd44780::Font::Font5x8,
        },
    );

    let mut lcd = pwr_hd44780::frontend::Buffered::new(&mut direct_lcd);

    // finally - print our text
    loop {
        lcd.clear();
        lcd.print("Hello World! :-)");
        lcd.render();
    }
}