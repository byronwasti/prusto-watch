#![feature(proc_macro)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f30x_hal as hal;
extern crate ls010b7dh01;
extern crate rn4870;
extern crate embedded_graphics as graphics;
extern crate panic_abort;
extern crate nb;

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
use graphics::fonts::{Font, Font6x8};
use graphics::transform::Transform;
use graphics::image::Image1BPP;

app! {
    device: hal::stm32f30x,

    resources: {
        static TOGGLE: bool = false;
        static TIME: u8 = 0;
        static STATE: State = State::Ble;
        static EXTI: hal::stm32f30x::EXTI;
        static RESET_BLE: bool = true;
        static REDRAW: bool = true;
        static DRAW_BUFFER: [u8; 16] = [32; 16];
        static BUFFER_POS: u8 = 0;

        // Late Resources
        static EXTCOMIN: display::Extcomin;
        static DISPLAY: display::Display;
        static BLE: ble::Ble;
    },

    tasks: {
        TIM7: {
            path: tick,
            resources: [TOGGLE, EXTCOMIN, DISPLAY],
        },

        SYS_TICK: {
            path: sys_tick,
            resources: [TOGGLE, EXTCOMIN, DISPLAY,
                TIME, BLE, RESET_BLE, STATE, REDRAW,
                DRAW_BUFFER],
        },

        USART1_EXTI25: {
            path: ble_message,
            resources: [BLE, DRAW_BUFFER, BUFFER_POS],
        },

        EXTI9_5: {
            enabled: true,
            priority: 1,
            path: exti9_5,
            resources: [STATE, EXTI],
        },

        EXTI15_10: {
            path: exti15_10,
            resources: [STATE, EXTI],
        },
    },
}

pub enum State {
    Ble,
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
    let mut delay = Delay::new(p.core.SYST, clocks);

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
    serial.listen(serial::Event::Rxne); // TODO: Serial interrupts?
    let mut ble = rn4870::Rn4870::new(serial, reset_ble);

    // Set the default values
    v5_en.set_high();
    display.enable();

    // Set up syscfg to link GPIO to EXTI
    p.device.SYSCFG.exticr3.modify(|_, w| unsafe {
        w.bits(0x11)
        /* This does not work
        w.exti8().bits(0b001) // Port b
            .exti9().bits(0b001) // Port b
        */
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
        EXTI: p.device.EXTI,
    }
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

fn ble_message(_t: &mut Threshold, mut r: USART1_EXTI25::Resources) {
    let res = r.BLE.read_raw();
    match res {
        Ok(n) => {
            if n < 32 {
                return
            }
            (*r.DRAW_BUFFER)[*r.BUFFER_POS as usize] = n;
            *r.BUFFER_POS += 1;
            if *r.BUFFER_POS == 16 {
                *r.BUFFER_POS = 0;
            }
        }
        Err(nb::Error::Other(_)) => {
            r.BLE.handle_error(|uart| { uart.clear_overflow_error(); } );
        }
        Err(nb::Error::WouldBlock) => {}
    }
}

fn exti9_5(_t: &mut Threshold, mut r: EXTI9_5::Resources) {
    if r.EXTI.pr1.read().pr8().bit_is_set() {
        r.EXTI.pr1.modify(|_, w| w.pr8().set_bit());
    }

    if r.EXTI.pr1.read().pr9().bit_is_set() {
        r.EXTI.pr1.modify(|_, w| w.pr9().set_bit());
        *r.STATE = State::Time;
    }
}

fn exti15_10(_t: &mut Threshold, mut r: EXTI15_10::Resources) {
    if r.EXTI.pr1.read().pr13().bit_is_set() {
        r.EXTI.pr1.modify(|_, w| w.pr13().set_bit());

        *r.STATE = State::Face;
    }
}

fn tick(_t: &mut Threshold, mut r: TIM7::Resources) {
}

fn sys_tick(_t: &mut Threshold, mut r: SYS_TICK::Resources) {
    let toggle = *r.TOGGLE;
    let extcomin = &mut *r.EXTCOMIN;

    if *r.RESET_BLE {
        r.BLE.hard_reset_on();
        *r.RESET_BLE = false;
    } else {
        r.BLE.hard_reset_off();
    }

    match *r.STATE {
        State::Ble => {
            r.DISPLAY.clear();
            //let s = String::from_utf8_lossy(&*r.DRAW_BUFFER);
            unsafe { 
                let s = &*(&*r.DRAW_BUFFER as *const [u8] as *const str); 
                r.DISPLAY.draw(Font6x8::render_str(s).translate((5, 50)).into_iter());
                r.DISPLAY.flush_buffer();
            }
        }
        State::Time => {
            *r.REDRAW = true;

            draw_time(&mut *r.DISPLAY, *r.TIME);
            *r.TIME += 1;
            if *r.TIME == 60 {
                *r.TIME = 0;
            }
        }
        State::Face => {
            if *r.REDRAW {
                draw_face(&mut *r.DISPLAY);
                *r.REDRAW = false;
            }
        }
    }


    // Toggle extcomin manually
    if toggle {
        (*extcomin).set_high();

    } else {
        (*extcomin).set_low();
    }

    *r.TOGGLE = !toggle;
}

fn draw_face(mut display: &mut display::Display) {
    display.clear();
    let bpp = Image1BPP::new(include_bytes!("../data/face_1bpp_neg.raw"), 120, 120)
        .translate((0, 0));
    display.draw(bpp.into_iter());
    display.flush_buffer();

}

fn draw_time(mut display: &mut display::Display, time: u8) {

    let values = [
        (125, 65), (124, 71), (123, 77), (122, 83), (119, 89),
        (116, 94), (113, 100), (109, 105), (105, 109), (100, 113),
        (95, 116), (89, 119), (83, 122), (77, 123), (71, 124),

        (65, 125), (59, 124), (53, 123), (47, 122), (41, 119),
        (36, 116), (30, 113), (25, 109), (21, 105), (17, 100),
        (14, 95), (11, 89), (8, 83), (7, 77), (6, 71),

        (5, 65), (6, 59), (7, 53), (8, 47), (11, 41),
        (14, 36), (17, 30), (21, 25), (25, 21), (30, 17),
        (35, 14), (41, 11), (47, 8), (53, 7), (59, 6),

        (65, 5), (71, 6), (77, 7), (83, 8), (89, 11),
        (94, 14), (100, 17), (105, 21), (109, 25), (113, 30),
        (116, 35), (119, 41), (122, 47), (123, 53), (124, 59),
    ];


    display.clear();
    display.draw(Line::new((65, 65), values[time as usize], 1).into_iter());
    display.flush_buffer();
}

fn draw_buffer(buffer: &[u8]) {
}
