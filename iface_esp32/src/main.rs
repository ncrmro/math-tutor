//! I2C Display example
//!
//! This example prints some text on an SSD1306-based
//! display (via I2C)
//!
//! The following wiring is assumed:
//! - SDA => GPIO1
//! - SCL => GPIO2

#![no_std]
#![no_main]

use embedded_graphics::mono_font::iso_8859_16::FONT_9X15;
use embedded_graphics::{
    mono_font::{
        ascii::{FONT_6X10, FONT_9X18_BOLD},
        MonoTextStyleBuilder,
    },
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Text},
};
use esp32c3_hal::{
    clock::ClockControl, gpio::IO, i2c::I2C, peripherals::Peripherals, prelude::*,
    timer::TimerGroup, Rtc,
};
use heapless::String;

use esp_backtrace as _;
use esp_println::println;
use nb::block;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

struct App {
    top_level_menu: i8,
}

#[entry]
fn main() -> ! {
    let mut app = App { top_level_menu: 0 };
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Instantiate and Create Handles for the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    // Disable the RTC and TIMG watchdog timers
    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // Create a new peripheral object with the described wiring
    // and standard I2C clock speed
    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio6,
        io.pins.gpio7,
        100u32.kHz(),
        &clocks,
    );

    // Initialize display
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    // Initialize button
    let button = io.pins.gpio0.into_pull_up_input();

    // Specify different text styles
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_9X15)
        .text_color(BinaryColor::On)
        .build();
    let text_style_selected = MonoTextStyleBuilder::new()
        .font(&FONT_9X15)
        .text_color(BinaryColor::Off)
        .background_color(BinaryColor::On)
        .build();

    loop {
        if button.is_low().unwrap() {
            if app.top_level_menu > 2 {
                app.top_level_menu = 0
            } else {
                app.top_level_menu += 1
            }
            println!("Current menu is {}", app.top_level_menu);
        }
        // Fill display bufffer with a centered text with two lines (and two text
        // styles)
        Text::with_alignment(
            "Numbers",
            display.bounding_box().center() + Point::new(0, -20),
            if app.top_level_menu == 0 {
                text_style_selected
            } else {
                text_style
            },
            Alignment::Center,
        )
        .draw(&mut display)
        .unwrap();

        Text::with_alignment(
            "Counting",
            display.bounding_box().center() + Point::new(0, 0),
            if app.top_level_menu == 1 {
                text_style_selected
            } else {
                text_style
            },
            Alignment::Center,
        )
        .draw(&mut display)
        .unwrap();

        Text::with_alignment(
            "Addition",
            display.bounding_box().center() + Point::new(0, 20),
            if app.top_level_menu == 2 {
                text_style_selected
            } else {
                text_style
            },
            Alignment::Center,
        )
        .draw(&mut display)
        .unwrap();

        // Write buffer to display
        display.flush().unwrap();
        // Clear display buffer
        display.clear(BinaryColor::Off).unwrap();
    }
}
