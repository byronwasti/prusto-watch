#![feature(proc_macro)]
#![no_std]

extern crate cortex_m_rtfm as rtfm;
extern crate stm32f303;

use rtfm::app;

app! {
    device: stm32f303,
}

fn init(_p: init::Peripherals) {
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

