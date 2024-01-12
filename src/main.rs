#![no_std]
#![no_main]

use esp32c6_hal::{clock::ClockControl, gpio::IO, peripherals::Peripherals, prelude::*, Delay, Rmt};
use esp_println::println;
use esp_hal_smartled::{smartLedBuffer, SmartLedsAdapter};
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
        let (r, g, b) = hsv_to_rgb(h + 20.0, s, v);
        //println!("h: {}, s: {}, v: {} -> r: {}, g: {}, b: {}", h, s, v, r, g, b);
        let data = [RGB8::new(r as u8, g as u8, b as u8); 1];
        led.write(data.iter().cloned()).unwrap();
        delay.delay_ms(30u32);
    }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (i32, i32, i32) {
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;

    let i = (h / 60.0) as i32 % 6;
    //println!("i: {}", i);
    let f = h / 60.0 - (h / 60.0) as i32 as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - (s * f));
    let t = v * (1.0 - (1.0 - f) * s);

    match i {
        0 => {
            r = v;
            g = t;
            b = p;
        }
        1 => {
            r = q;
            g = v;
            b = p;
        }
        2 => {
            r = p;
            g = v;
            b = t;
        }
        3 => {
            r = p;
            g = q;
            b = v;
        }
        4 => {
            r = t;
            g = p;
            b = v;
        }
        5 => {
            r = v;
            g = p;
            b = q;
        }
        _ => {}
    }

    ((r * 255.0) as i32, (g * 255.0) as i32, (b * 255.0) as i32)
}