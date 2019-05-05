#![no_std]
#![no_main]

extern crate panic_halt;

mod bombs;
mod landing_gear;
mod lights;
mod receiver;
mod servo;

use cortex_m::asm;
use cortex_m_rt::{entry, exception, ExceptionFrame};

use f3::hal::prelude::*;
use f3::hal::stm32f30x;
use f3::led::Leds;

use self::bombs::Bombs;
use self::landing_gear::LandingGear;
use self::lights::Lights;

#[entry]
fn main() -> ! {
    let mut bombs = Bombs::default();
    let mut landing_gear = LandingGear::default();
    let mut lights = Lights::default();

    // Turn on all LEDs, for fun!
    let p = stm32f30x::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();
    let gpioe = p.GPIOE.split(&mut rcc.ahb);
    let mut leds = Leds::new(gpioe);
    for led in leds.iter_mut() {
        led.on();
    }

    loop {
        bombs.update();
        landing_gear.update();
        lights.update();
    }
}

#[exception]
fn DefaultHandler(_irqn: i16) {
    asm::bkpt();
}

#[exception]
fn HardFault(_ef: &ExceptionFrame) -> ! {
    asm::bkpt();

    loop {}
}
