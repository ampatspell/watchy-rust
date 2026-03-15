use defmt::info;
use embassy_time::{Duration, Timer};
use embedded_graphics::{
    Drawable,
    mono_font::{MonoTextStyle, iso_8859_5::FONT_9X18_BOLD},
    prelude::*,
    primitives::{PrimitiveStyle, StyledDrawable},
    text::Text,
};
use epd_waveshare::{color::Color, prelude::RefreshLut};
use no_std_strings::{str16};

use crate::{
    display::{DisplayPins, display::Display},
};

#[embassy_executor::task]
pub async fn display_task(pins: DisplayPins) {
    info!("start display_task");

    let mut display = Display::new(pins);

    let clear = |display: &mut Display| {
        let style = PrimitiveStyle::with_fill(Color::White);
        display.bounding_box().draw_styled(&style, display);
    };

    display.set_lut(RefreshLut::Full);
    clear(&mut display);

    let style = MonoTextStyle::new(&FONT_9X18_BOLD, Color::Black);
    let  button = str16::from("Watchy v1");

    display.update_and_display();
    display.set_lut(RefreshLut::Full);

    loop {
        clear(&mut display);

        Text::new(button.to_str(), Point::new(10, 25), style)
            .draw(&mut display)
            .unwrap();

        display.update_and_display();

        loop {
            Timer::after(Duration::from_secs(60)).await;
        }
    }
}
