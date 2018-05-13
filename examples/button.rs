#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate panic_abort;
extern crate stm32f30x_hal as hal;

use hal::prelude::*;
use hal::stm32f30x;
use hal::spi::Spi;

fn main() {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);
    
    // Set up Pins
    let mut button = gpiob.pb8
        .into_pull_up_input(&mut gpiob.moder, &mut gpiob.pupdr);
    let mut led = gpioc.pc13
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

    loop {
        if button.is_low() {
            led.set_low();
        } else {
            led.set_high();
        }
    }
}

