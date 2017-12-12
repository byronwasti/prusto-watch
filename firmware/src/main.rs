#![feature(proc_macro)]
#![no_std]

extern crate cortex_m_rtfm as rtfm;
extern crate stm32f30x;

use core::u16;
use rtfm::{app, Threshold};

app! {
    device: stm32f30x,

    resources: {
        static ON: bool = false;
    },

    tasks: {
        TIM7: {
            path: toggle,
            resources: [ON, GPIOE],
        },
    },
}

fn init(p: init::Peripherals, _r: init::Resources) {
    let gpioe = p.GPIOE;
    let rcc = p.RCC;
    let tim7 = p.TIM7;

    rcc.ahbenr.modify(|_, w| w.iopeen().enabled());
    rcc.apb1enr.modify(|_, w| w.tim7en().enabled());

    gpioe.moder.modify(|_, w| w.moder9().output());

    let ratio = 8_000_000;
    let psc = ((ratio - 1) /(u16::MAX as u32)) as u16;
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
    // Toggle our state
    **r.ON = !**r.ON;

    let gpioe = &**r.GPIOE;
    if **r.ON {
        gpioe.bsrr.write(|w| w.bs9().set());
    } else {
        gpioe.bsrr.write(|w| w.br9().reset());
    }
}

