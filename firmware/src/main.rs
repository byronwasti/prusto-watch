// examples/blinky.rs

#![feature(used)]
#![no_std]

// version = "0.2.0", default-features = false
extern crate cortex_m;
extern crate cortex_m_rt;
//extern crate cortex_m_semihosting;
extern crate stm32f303;

use core::fmt::Write;
use core::u16;

use cortex_m::asm;
//use cortex_m_semihosting::hio;
use stm32f303::{GPIOA, RCC, TIM7};

mod frequency {
    /// Frequency of APB1 bus (TIM7 is connected to this bus)
    pub const APB1: u32 = 8_000_000;
}

/// Timer frequency
const FREQUENCY: u32 = 1;

#[inline(never)]
fn main() {
    // Critical section, this closure is non-preemptable
    cortex_m::interrupt::free(
        |cs| {
            // INITIALIZATION PHASE
            // Exclusive access to the peripherals
            let gpioa = GPIOA.borrow(cs);
            let rcc = RCC.borrow(cs);
            let tim7 = TIM7.borrow(cs);

            // Power up the relevant peripherals
            rcc.ahbenr.modify(|_, w| w.iopaen().enabled());
            rcc.apb1enr.modify(|_, w| w.tim7en().enabled());

            // Configure the pin PE9 as an output pin
            gpioa.moder.modify(|_, w| w.moder9().output());

            // Configure TIM7 for periodic timeouts
            let ratio = frequency::APB1 / FREQUENCY;
            let psc = ((ratio - 1) / (u16::MAX) as u32) as u16;
            tim7.psc.write(|w| w.psc().bits(psc));
            let arr = (ratio / (psc + 1) as u32) as u16;
            tim7.arr.write(|w| w.arr().bits(arr));
            tim7.cr1.write(|w| w.opm().continuous());

            // Start the timer
            tim7.cr1.modify(|_, w| w.cen().enabled());

            // APPLICATION LOGIC
            let mut state = false;
            loop {
                // Wait for an update event
                while tim7.sr.read().uif().is_no_update() {}

                // Clear the update event flag
                tim7.sr.modify(|_, w| w.uif().clear());

                // Toggle the state
                state = !state;

                //let mut stdout = hio::hstdout().unwrap();

                // Blink the LED
                if state {
                    gpioa.bsrr.write(|w| w.br9().reset());
                    //writeln!(stdout, "On").unwrap();
                } else {
                    gpioa.bsrr.write(|w| w.bs9().set());
                    //writeln!(stdout, "Off").unwrap();
                }
            }
        },
    );
}

// This part is the same as before
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
