#![feature(proc_macro)]
#![no_std]

extern crate cortex_m_rtfm as rtfm;
//extern crate stm32f30x;
extern crate stm32f30x_hal as hal;
extern crate cortex_m_semihosting;

use core::u16;
use rtfm::{app, Threshold};
use hal::stm32f30x;
use hal::serial::{Serial, Rx, Tx};

use stm32f30x::{GPIOA};

use core::fmt::Write;
use cortex_m_semihosting::hio;

app! {
    device: stm32f30x,

    resources: {
        static VAL: bool = false;
        static COUNTER: u32 = 0;
        static LED: GPIOA;
    },

    idle: {
        resources: [VAL],
    },

    tasks: {
        TIM7: {
            path: toggle,
            resources: [VAL, COUNTER, LED],
        },
    },
}

fn init(p: init::Peripherals, r: init::Resources) -> init::LateResources {
    let gpioa = p.device.GPIOA;
    let rcc = p.device.RCC;
    let tim7 = p.device.TIM7;

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

    init::LateResources {
        LED: gpioa,
    }
}

fn idle(t: &mut Threshold, r: idle::Resources) -> ! {
    loop {
        rtfm::wfi();
    }
}

fn toggle(t: &mut Threshold, mut r: TIM7::Resources) {
    if *r.COUNTER > 10000 {
        *r.VAL = !*r.VAL;
        *r.COUNTER = 0;

        let gpioa = r.LED;
        if *r.VAL {
            gpioa.bsrr.write(|w| w.bs9().set());
        } else {
            gpioa.bsrr.write(|w| w.br9().reset());
        }
    } else {
        *r.COUNTER += 1;
    }
}

