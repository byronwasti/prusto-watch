#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_abort;
extern crate stm32f30x_hal as hal;

use hal::prelude::*;
use hal::stm32f30x;
use hal::delay::Delay;
use hal::usb::Usb;

fn main() {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Set up delay
    let mut delay = Delay::new(cp.SYST, clocks);

    // Set up USB
    let usb = Usb::new(dp.USB_FS, &mut rcc.apb1, &mut delay);

    loop {
    }
}

#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    loop {}
}

