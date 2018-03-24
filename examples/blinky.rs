#![no_std]

extern crate cortex_m;
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
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    let mut led1 = gpioc
        .pc13
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

    let mut led2 = gpiob
        .pb8
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut led3 = gpiob
        .pb0
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        led1.set_high();
        led2.set_low();
        led3.set_high();
        delay.delay_ms(1_000_u16);
        led2.set_high();
        led1.set_low();
        led3.set_low();
        delay.delay_ms(1_000_u16);
    }
}
