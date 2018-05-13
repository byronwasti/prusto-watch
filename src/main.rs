#![feature(proc_macro)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f30x_hal as hal;
extern crate ls010b7dh01;
extern crate rn4870;
extern crate embedded_graphics as graphics;
extern crate panic_abort;

mod display;
mod ble;

use cortex_m::asm;
use cortex_m::peripheral::syst::SystClkSource;
use rtfm::{app, Threshold};
use hal::prelude::*;
use hal::timer;
use hal::timer::Timer;
use hal::spi::Spi;
use hal::serial;
use hal::serial::Serial;
use hal::delay::Delay;
use hal::gpio::{gpiob, gpioc, Input, Output, PullUp, PushPull, AF7};
use ls010b7dh01::Ls010b7dh01;
use graphics::prelude::*;
use graphics::primitives::{Circle, Line, Rect};

app! {
    device: hal::stm32f30x,

    resources: {
        static TOGGLE: bool = false;
        static TIME: u8 = 0;
        static STATE: State = State::Time;

        // Late Resources
        static EXTCOMIN: display::Extcomin;
        static DISPLAY: display::Display;
        static BLE: ble::Ble;
        //static LED: gpioc::PC13<Output<PushPull>>;
        //static BUTTON: gpiob::PB8<Input<PullUp>>;
    },

    tasks: {
        TIM7: {
            path: tick,
            resources: [TOGGLE, EXTCOMIN, DISPLAY],
        },

        SYS_TICK: {
            path: sys_tick,
            resources: [TOGGLE, EXTCOMIN, DISPLAY, TIME],
        },

        USART1_EXTI25: {
            path: ble_message,
            resources: [BLE],
        },

        EXTI9_5: {
            enabled: true,
            priority: 1,
            path: exti9_5,
            resources: [STATE],
            //resources: [LED, BUTTON],
            //resources: [LED],
        }
    },
}

pub enum State {
    Time,
    Face,
}

fn init(mut p: init::Peripherals, _r: init::Resources) -> init::LateResources {
    let mut rcc = p.device.RCC.constrain();
    let mut flash = p.device.FLASH.constrain();
    let mut gpioa = p.device.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = p.device.GPIOB.split(&mut rcc.ahb);
    let mut gpioc = p.device.GPIOC.split(&mut rcc.ahb);

    // Enable the syscfg
    rcc.apb2.enr().modify(|_, w| w.syscfgen().enabled());
    rcc.apb2.rstr().modify(|_, w| w.syscfgrst().set_bit());
    rcc.apb2.rstr().modify(|_, w| w.syscfgrst().clear_bit());

    // Enable systick
    p.core.SYST.set_clock_source(SystClkSource::Core);
    p.core.SYST.set_reload(16_000_000);
    p.core.SYST.enable_interrupt();
    p.core.SYST.enable_counter();

    // Set up our clocks & timer & delay
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut timer = Timer::tim7(p.device.TIM7, 1.hz(), clocks, &mut rcc.apb1);
    //timer.listen(timer::Event::TimeOut);
    let delay = Delay::new(p.core.SYST, clocks);

    // Set up our GPIO pins
    let disp_en = gpiob.pb2.into_push_pull_output(
        &mut gpiob.moder,
        &mut gpiob.otyper,
    );
    let extcomin = gpiob.pb1.into_push_pull_output(
        &mut gpiob.moder,
        &mut gpiob.otyper,
    );
    let cs = gpiob.pb0.into_push_pull_output(
        &mut gpiob.moder,
        &mut gpiob.otyper,
    );
    let mut v5_en = gpioa.pa3.into_push_pull_output(
        &mut gpioa.moder,
        &mut gpioa.otyper,
    );
    let reset_ble = gpiob.pb5.into_push_pull_output(
        &mut gpiob.moder,
        &mut gpiob.otyper,
    );
    /*
    let mut led = gpioc.pc13.into_push_pull_output(
        &mut gpioc.moder,
        &mut gpioc.otyper,
    );
    */
    let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let tx = gpiob.pb6.into_af7(&mut gpiob.moder, &mut gpiob.afrl);
    let rx = gpiob.pb7.into_af7(&mut gpiob.moder, &mut gpiob.afrl);
    let button_1 = gpiob.pb8.into_pull_up_input(
        &mut gpiob.moder,
        &mut gpiob.pupdr,
    );
    let button_2 = gpiob.pb9.into_pull_up_input(
        &mut gpiob.moder,
        &mut gpiob.pupdr,
    );
    let button_3 = gpioc.pc13.into_pull_up_input(
        &mut gpioc.moder,
        &mut gpioc.pupdr,
    );

    // Set up our display
    let mode = ls010b7dh01::MODE;
    let spi = Spi::spi1(
        p.device.SPI1,
        (sck, miso, mosi),
        mode,
        1.mhz(),
        clocks,
        &mut rcc.apb2,
    );
    let mut display = Ls010b7dh01::new(spi, cs, disp_en);

    // Set up our BLE
    let mut serial = Serial::usart1(
        p.device.USART1,
        (tx, rx),
        115_200.bps(),
        clocks,
        &mut rcc.apb2,
    );
    serial.listen(serial::Event::Rxne);
    let mut ble = rn4870::Rn4870::new(serial, reset_ble);

    // Set the default values
    v5_en.set_high();
    display.enable();
    //led.set_high();

    // Set up syscfg to link GPIO to EXTI
    p.device.SYSCFG.exticr3.modify(|_, w| unsafe {
        w.exti8().bits(0b001) // Port b
            .exti9().bits(0b001) // Port b
    });
    p.device.SYSCFG.exticr4.modify(|_, w| unsafe {
        w.exti13().bits(0b010) // Port c
    });
    p.device.EXTI.imr1.modify(|_, w| {
        w.mr8().set_bit().mr9().set_bit().mr13().set_bit()
    });

    p.device.EXTI.ftsr1.modify(|_, w| {
        w.tr8().set_bit().tr9().set_bit().tr13().set_bit()
    });

    init::LateResources {
        DISPLAY: display,
        EXTCOMIN: extcomin,
        BLE: ble,
    }
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

fn ble_message(_t: &mut Threshold, mut r: USART1_EXTI25::Resources) {
    // TODO
}

fn exti9_5(_t: &mut Threshold, mut r: EXTI9_5::Resources) {
    asm::bkpt();
}

fn tick(_t: &mut Threshold, mut r: TIM7::Resources) {
    /*
    let toggle = *r.TOGGLE;
    let extcomin = &mut *r.EXTCOMIN;
    let display = &mut *r.DISPLAY;

    if toggle {
        (*extcomin).set_high();

        (*display).clear();
        (*display).draw(Line::new((0, 0), (65, 65), 1).into_iter());
        (*display).flush_buffer();
    } else {
        (*extcomin).set_low();
    }

    *r.TOGGLE = !toggle;
    */
}

fn sys_tick(_t: &mut Threshold, mut r: SYS_TICK::Resources) {
    let toggle = *r.TOGGLE;
    let extcomin = &mut *r.EXTCOMIN;
    //let led = &mut *r.LED;

    draw_time(&mut *r.DISPLAY, *r.TIME);
    *r.TIME += 1;
    if *r.TIME == 60 {
        *r.TIME = 0;
    }

    if toggle {
        (*extcomin).set_high();
        //(*led).set_high();

    } else {
        //(*led).set_low();
        (*extcomin).set_low();
    }

    *r.TOGGLE = !toggle;
}

fn draw_face(mut display: &mut display::Display) {}

fn draw_time(mut display: &mut display::Display, time: u8) {

    let values = [
        (125, 65),
        (124, 71),
        (123, 77),
        (122, 83),
        (119, 89),
        (116, 94),
        (113, 100),
        (109, 105),
        (105, 109),
        (100, 113),
        (95, 116),
        (89, 119),
        (83, 122),
        (77, 123),
        (71, 124),

        (65, 125),
        (59, 124),
        (53, 123),
        (47, 122),
        (41, 119),
        (36, 116),
        (30, 113),
        (25, 109),
        (21, 105),
        (17, 100),
        (14, 95),
        (11, 89),
        (8, 83),
        (7, 77),
        (6, 71),

        (5, 65),
        (6, 59),
        (7, 53),
        (8, 47),
        (11, 41),
        (14, 36),
        (17, 30),
        (21, 25),
        (25, 21),
        (30, 17),
        (35, 14),
        (41, 11),
        (47, 8),
        (53, 7),
        (59, 6),

        (65, 5),
        (71, 6),
        (77, 7),
        (83, 8),
        (89, 11),
        (94, 14),
        (100, 17),
        (105, 21),
        (109, 25),
        (113, 30),
        (116, 35),
        (119, 41),
        (122, 47),
        (123, 53),
        (124, 59),
    ];


    display.clear();
    display.draw(Line::new((65, 65), values[time as usize], 1).into_iter());
    display.flush_buffer();
}
