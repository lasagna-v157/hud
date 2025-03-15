use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

fn main(){
	use embedded_graphics::{
	    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
	    pixelcolor::BinaryColor,
	    prelude::*,
	    text::{Baseline, Text},
	};
	use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

	let interface = I2CDisplayInterface::new(i2c);
	let mut display = Ssd1306::new(
	    interface,
	    DisplaySize128x64,
	    DisplayRotation::Rotate0,
	).into_buffered_graphics_mode();
	display.init().unwrap();

	let text_style = MonoTextStyleBuilder::new()
	    .font(&FONT_6X10)
	    .text_color(BinaryColor::On)
	    .build();

	Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
	    .draw(&mut display)
	    .unwrap();

	Text::with_baseline("Hello Rust!", Point::new(0, 16), text_style, Baseline::Top)
	    .draw(&mut display)
	    .unwrap();

	display.flush().unwrap();	
}

