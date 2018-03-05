#![no_std]

extern crate cortex_m;
extern crate stm32f30x_hal as hal;
#[macro_use(block)]
extern crate nb;
extern crate cortex_m_semihosting as semihosting;

use cortex_m::asm;
use hal::prelude::*;
use hal::serial::Serial;
use hal::stm32f30x;
use hal::delay::Delay;

use core::fmt::Write;
use semihosting::hio;

fn main() {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = stm32f30x::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb);

    // clock configuration using the default settings (all clocks run at 8 MHz)
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Set up Reset BLE line
    let mut reset_ble = gpiob
        .pb12
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut delay = Delay::new(cp.SYST, clocks);
    reset_ble.set_low();
    delay.delay_ms(1_000u16);
    reset_ble.set_high();
    
    loop {
    }

    asm::bkpt()
}

