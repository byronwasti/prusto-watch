#![no_std]

extern crate cortex_m;
extern crate stm32f30x_hal as hal;
extern crate embedded_hal;
#[macro_use(block)]
extern crate nb;

// TODO Remove this dependancy
use embedded_hal::spi::{Mode, Phase, Polarity};

use hal::prelude::*;
use hal::spi::Spi;
use hal::stm32f30x;

fn main() {
    let p = stm32f30x::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);

    // clock configuration using the default settings (all clocks run at 8 MHz)
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Set up SPI
    let pa5 = gpioa
        .pa5
        .into_af5(&mut gpioa.moder, &mut gpioa.afrl); // SCK

    let pa6 = gpioa
        .pa6
        .into_af5(&mut gpioa.moder, &mut gpioa.afrl); // MISO

    let pa7 = gpioa
        .pa7
        .into_af5(&mut gpioa.moder, &mut gpioa.afrl); // MOSI

    let mode = Mode { polarity: Polarity::IdleLow, phase: Phase::CaptureOnFirstTransition };
    let mut spi = Spi::spi1(p.SPI1, (pa5, pa6, pa7), mode, 100.khz(), clocks, &mut rcc.apb2);

    // Send data
    block!(spi.send(b'$')).ok();
}
