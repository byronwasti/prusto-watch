#![feature(proc_macro)]
#![no_std]

extern crate cortex_m_rtfm as rtfm;
extern crate stm32f30x_hal as hal;
extern crate embedded_hal as hal2;
extern crate cortex_m_semihosting;

// TODO Remove this dependancy
use hal2::spi::{Mode, Phase, Polarity};

use core::u16;
use rtfm::{app, Threshold};
use hal::prelude::*;
use hal::stm32f30x;
use hal::stm32f30x::{SPI1};
use hal::timer::{Timer, Event};
use hal::spi::{Spi};
use hal::gpio;
use hal::gpio::{gpioa, Output, PushPull};
use hal::gpio::gpioa::{PA5, PA6, PA7};
//use hal::serial::{Serial, Rx, Tx};

use stm32f30x::{GPIOA};

use core::fmt::Write;
use cortex_m_semihosting::hio;

app! {
    device: stm32f30x,

    resources: {
        static VAL: bool = false;
        static COUNTER: u32 = 0;

        // Late Resources
        static LED: gpioa::PA9<Output<PushPull>>;
        static SPI: Spi<SPI1, (PA5<gpio::AF5>, PA6<gpio::AF5>, PA7<gpio::AF5>)>;
    },

    idle: {
        resources: [VAL],
    },

    tasks: {
        TIM7: {
            path: toggle,
            resources: [VAL, COUNTER, LED, SPI],
        },
    },
}

fn init(p: init::Peripherals, r: init::Resources) -> init::LateResources {
    let mut rcc = p.device.RCC.constrain();
    let mut flash = p.device.FLASH.constrain();
    let mut gpioa = p.device.GPIOA.split(&mut rcc.ahb);

    // Set up our timer
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut timer = Timer::tim7(p.device.TIM7, 1.hz(), clocks, &mut rcc.apb1);
    timer.listen(Event::TimeOut);

    // Set up SPI
    let mut pa5 = gpioa
        .pa5
        .into_af5(&mut gpioa.moder, &mut gpioa.afrl); // SCK

    let mut pa6 = gpioa
        .pa6
        .into_af5(&mut gpioa.moder, &mut gpioa.afrl); // MISO

    let mut pa7 = gpioa
        .pa7
        .into_af5(&mut gpioa.moder, &mut gpioa.afrl); // MOSI


    let mode = Mode { polarity: Polarity::IdleLow, phase: Phase::CaptureOnFirstTransition };
    let spi = Spi::spi1(p.device.SPI1, (pa5, pa6, pa7), mode, 100.khz(), clocks, &mut rcc.apb2);

    // Set up our debug LED
    let led = gpioa
        .pa9
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    init::LateResources {
        LED: led,
        SPI: spi,
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

        // Toggle LED
        if *r.VAL {
            r.LED.set_high();
        } else {
            r.LED.set_low();
        }

        // Send on SPI
        r.SPI.send(0xBF);
    } else {
        *r.COUNTER += 1;
    }
}

