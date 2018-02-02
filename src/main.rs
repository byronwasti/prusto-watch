#![feature(proc_macro)]
#![no_std]

extern crate cortex_m_rtfm as rtfm;
extern crate stm32f30x_hal as hal;
extern crate cortex_m_semihosting;

use core::u16;
use rtfm::{app, Threshold};
use hal::prelude::*;
use hal::stm32f30x;
use hal::timer::{Timer, Event};
//use hal::serial::{Serial, Rx, Tx};

use stm32f30x::{GPIOA};

use core::fmt::Write;
use cortex_m_semihosting::hio;

app! {
    device: stm32f30x,

    resources: {
        static VAL: bool = false;
        static COUNTER: u32 = 0;
        static LED: hal::gpio::gpioa::PA9<hal::gpio::Output<hal::gpio::PushPull>>;
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
    let mut rcc = p.device.RCC.constrain();
    let mut flash = p.device.FLASH.constrain();
    let mut gpioa = p.device.GPIOA.split(&mut rcc.ahb);

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut timer = Timer::tim7(p.device.TIM7, 1.hz(), clocks, &mut rcc.apb1);
    timer.listen(Event::TimeOut);


    let mut led = gpioa
        .pa9
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    init::LateResources {
        LED: led,
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

        //let gpioa = r.LED;
        if *r.VAL {
            r.LED.set_high();
            //gpioa.bsrr.write(|w| w.bs9().set());
        } else {
            r.LED.set_low();
            //gpioa.bsrr.write(|w| w.br9().reset());
        }
    } else {
        *r.COUNTER += 1;
    }
}

