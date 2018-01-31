#![feature(proc_macro)]
#![no_std]

extern crate cortex_m_rtfm as rtfm;
extern crate stm32f30x;
extern crate cortex_m_semihosting;

use core::u16;
use rtfm::{app, Threshold};

use core::fmt::Write;
use cortex_m_semihosting::hio;

app! {
    device: stm32f30x,

    resources: {
        static ON: bool = false;
        static COUNTER: u32 = 0;
    },

    tasks: {
        TIM7: {
            path: toggle,
            resources: [ON, COUNTER, GPIOA],
        },
    },
}

fn init(p: init::Peripherals, _r: init::Resources) {
    let gpioa = p.GPIOA;
    let rcc = p.RCC;
    let tim7 = p.TIM7;

    rcc.ahbenr.modify(|_, w| w.iopaen().enabled());
    rcc.apb1enr.modify(|_, w| w.tim7en().enabled());

    gpioa.moder.modify(|_, w| w.moder9().output());

    let ratio = 8_000_000;
    //let psc = ((ratio - 1) /(u16::MAX as u32)) as u16;
    let psc = u16::MAX - 100;
    tim7.psc.write(|w| w.psc().bits(psc));
    let arr = (ratio / ((psc + 1) as u32)) as u16;
    tim7.arr.write(|w| w.arr().bits(arr));
    tim7.cr1.write(|w| w.opm().continuous());

    tim7.dier.write(|w| w.uie().bit(true));

    tim7.cr1.modify(|_, w| w.cen().enabled());
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

fn toggle(_t: &mut Threshold, r: TIM7::Resources) {
    if (**r.COUNTER > 10000) {
        // Toggle our state
        **r.ON = !**r.ON;
        **r.COUNTER = 0;
        let gpioa = &**r.GPIOA;
        if **r.ON {
            gpioa.bsrr.write(|w| w.bs9().set());
        } else {
            gpioa.bsrr.write(|w| w.br9().reset());
        }
    } else {
        **r.COUNTER += 1;
    }
}

