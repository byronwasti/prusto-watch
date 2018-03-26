#![no_std]

extern crate cortex_m;
extern crate stm32f30x_hal as hal;
extern crate rn4870;

use cortex_m::asm;
use hal::prelude::*;
use hal::serial::Serial;
use hal::stm32f30x;
use hal::delay::Delay;

fn main() {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = stm32f30x::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb);

    // clock configuration using the default settings (all clocks run at 8 MHz)
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Set up serial
    let tx = gpiob
        .pb10
        .into_af7(&mut gpiob.moder, &mut gpiob.afrh);

    let rx = gpiob
        .pb11
        .into_af7(&mut gpiob.moder, &mut gpiob.afrh);

    let serial = Serial::usart3(p.USART3, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb1);

    // Set up Reset BLE line
    let reset_ble = gpiob
        .pb12
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    // Set up a delay
    let mut delay = Delay::new(cp.SYST, clocks);

    // Set up BLE
    let mut ble = rn4870::Rn4870::new(serial, reset_ble);

    let result = ble.hard_reset(&mut delay);
    //if result.is_err() { panic!("Error"); }

    let result = ble.enter_cmd_mode();
    //if result.is_err() { panic!("Error"); }

    let result = ble.set_name("byron");
    //if result.is_err() { panic!("Error"); }

    let result = ble.set_default_services(0);
    //if result.is_err() { panic!("Error"); }

    let result = ble.enter_data_mode();
    //if result.is_err() { panic!("Error"); }

    // Echo response
    loop {
        match ble.read_raw() {
            Ok(val) => {
                ble.send_raw(val);
            },
            Err(hal::serial::Error::Overrun) => {
                ble.handle_error(|uart| { uart.clear_overflow_error(); } );
            },
            _ => panic!(),
        }
    }

    // Break
    asm::bkpt()
}

