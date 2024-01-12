#![no_std]
#![no_main]

use esp32c6_hal::{clock::ClockControl, gpio::IO, peripherals::Peripherals, prelude::*, Delay, Rmt};
use esp_backtrace as _;
use esp_println::println;
use esp_hal_smartled::{smartLedBuffer, SmartLedsAdapter};
use palette::{FromColor, Hsv, Srgb};
use smart_leds::RGB8;
use smart_leds::SmartLedsWrite;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    println!("Hello world!");

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let rmt = Rmt::new(peripherals.RMT, 80u32.MHz(), &clocks).unwrap();
    let rmt_buffer = smartLedBuffer!(1);
    let mut led = SmartLedsAdapter::new(rmt.channel0, io.pins.gpio15, rmt_buffer);
    led.write([RGB8::default(); 1].into_iter()).unwrap();

    let mut h: f32 = 0.0;
    let s: f32 = 1.0;
    let v: f32 = 0.01;
    loop {
        //println!("Loop...");
        h += 2.0;
        if h > 360.0 {
            h = 0.0;
        }
        let rgb = Srgb::from_color(Hsv::new(h + 20.0, s, v));
        let (r, g, b) = ((rgb.red * 255.0) as u8, (rgb.green * 255.0) as u8, (rgb.blue * 255.0) as u8);
        //println!("h: {}, s: {}, v: {} -> r: {}, g: {}, b: {}", h, s, v, r, g, b);
        let data = [RGB8::new(r, g, b); 1];
        led.write(data.iter().cloned()).unwrap();
        delay.delay_ms(20u32);
    }
}
