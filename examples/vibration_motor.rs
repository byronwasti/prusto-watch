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

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let mut motor = gpioa
        .pa0
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        motor.set_high();
        delay.delay_ms(1_000_u16);
        motor.set_low();
        delay.delay_ms(1_000_u16);
    }
}

