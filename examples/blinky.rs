#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate panic_abort;
extern crate stm32f30x_hal as hal;

use hal::prelude::*;
use hal::stm32f30x;
use hal::delay::Delay;

fn main() {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);

    let mut led1 = gpioc
        .pc13
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        led1.set_high();
        delay.delay_ms(1_000_u16);
        led1.set_low();
        delay.delay_ms(1_000_u16);
    }
}

