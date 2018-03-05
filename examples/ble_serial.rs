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

    let tx = gpiob
        .pb10
        .into_af7(&mut gpiob.moder, &mut gpiob.afrh);

    let rx = gpiob
        .pb11
        .into_af7(&mut gpiob.moder, &mut gpiob.afrh);

    let mut serial = Serial::usart3(p.USART3, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb1);
    //let (mut tx, mut rx) = serial.split();

    // Set up Reset BLE line
    let mut reset_ble = gpiob
        .pb12
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut delay = Delay::new(cp.SYST, clocks);
    reset_ble.set_low();
    delay.delay_ms(1_000u16);
    reset_ble.set_high();
    //let received = block!(rx.read()).unwrap();
    //let received2 = block!(rx.read()).unwrap();
    //let received3 = block!(rx.read()).unwrap();

    //let received = block!(rx.read());
    //let received2 = block!(rx.read());
    //let received3 = block!(rx.read());
    //writeln!(hio::hstdout().unwrap(), "{:?}{:?}{:?}", received, received2, received3).unwrap();
    delay.delay_ms(1_000u16);
    reset_ble.set_low();
    delay.delay_ms(1_000u16);
    reset_ble.set_high();
    delay.delay_ms(1_000u16);

    // Clear our overflow error
    let _ = serial.clear_overflow_error();

    block!(serial.write(b'$')).ok();
    block!(serial.write(b'$')).ok();
    block!(serial.write(b'$')).ok();

    let _ = block!(serial.read()); // C
    let _ = block!(serial.read()); // M
    let _ = block!(serial.read()); // D
    let _ = block!(serial.read()); // ' '
    let _ = serial.clear_overflow_error();

    block!(serial.write(b'S')).ok();
    block!(serial.write(b'S')).ok();
    block!(serial.write(b',')).ok();
    block!(serial.write(b'C')).ok();
    block!(serial.write(b'0')).ok();
    block!(serial.write(b'\r')).ok();

    let rec1 = block!(serial.read()).unwrap_or(serial.clear_overflow_error());
    if rec1 != b'A' {
        writeln!(hio::hstdout().unwrap(), "{}", rec1).unwrap();
    }

    let _ = serial.clear_overflow_error();

    // Write name
    block!(serial.write(b'S')).ok();
    block!(serial.write(b'-')).ok();
    block!(serial.write(b',')).ok();
    block!(serial.write(b'B')).ok();
    block!(serial.write(b'Y')).ok();
    block!(serial.write(b'R')).ok();
    block!(serial.write(b'O')).ok();
    block!(serial.write(b'\r')).ok();

    let rec1 = block!(serial.read()).unwrap_or(serial.clear_overflow_error());
    if rec1 != b'A' {
        writeln!(hio::hstdout().unwrap(), "{}", rec1).unwrap();
    }

    // Reboot module
    block!(serial.write(b'R')).ok();
    block!(serial.write(b',')).ok();
    block!(serial.write(b'1')).ok();
    block!(serial.write(b'\n')).ok();

    // Leave config mode
    /*
    block!(serial.write(b'-')).ok();
    block!(serial.write(b'-')).ok();
    block!(serial.write(b'-')).ok();
    block!(serial.write(b'\r')).ok();
    */


    //writeln!(hio::hstdout().unwrap(), "{}", rec1).unwrap();
    asm::bkpt()
}

