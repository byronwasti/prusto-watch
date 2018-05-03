#![feature(used)]
#![no_std]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f30x_hal as hal;
extern crate ls010b7dh01;
extern crate panic_abort;
extern crate embedded_graphics as graphics;

use cortex_m::asm;
use hal::prelude::*;
use hal::spi::Spi;
use hal::stm32f30x;
use hal::delay::Delay;
use ls010b7dh01::Ls010b7dh01;

use graphics::Drawing;
use graphics::primitives::{Circle, Line, Rect};
use graphics::fonts::{Font, Font6x8};
use graphics::transform::Transform;

fn main() {
    //asm::bkpt();

    let cp = cortex_m::Peripherals::take().unwrap();
    let p = stm32f30x::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb);

    // clock configuration using the default settings (all clocks run at 8 MHz)
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Set up delay
    let mut delay = Delay::new(cp.SYST, clocks);

    // Set up DISP_EN (Active high)
    let mut disp_en = gpiob
        .pb2
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper); // DISP_EN
    disp_en.set_low();

    let mut extcomin = gpiob
        .pb1
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    // Set up our CS (Active high)
    let mut cs = gpiob
        .pb0
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper); // CS
    cs.set_low();

    // Set up 5V_en
    let mut v5_en = gpioa
        .pa3
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper); // 5V_en
    v5_en.set_low();
    v5_en.set_high();

    // Set up SPI
    let pa5 = gpioa
        .pa5
        .into_af5(&mut gpioa.moder, &mut gpioa.afrl); // SCK

    let pa6 = gpioa
        .pa6
        .into_af5(&mut gpioa.moder, &mut gpioa.afrl); // MISO

    let pa7 = gpioa
        .pa7
        .into_af5(&mut gpioa.moder, &mut gpioa.afrl); // MOSI

    let mode = ls010b7dh01::MODE;
    let spi = Spi::spi1(p.SPI1, (pa5, pa6, pa7), mode, 1.mhz(), clocks, &mut rcc.apb2);

    // Driver
    asm::bkpt();
    let mut display = Ls010b7dh01::new(spi, cs, disp_en);
    asm::bkpt();

    display.disable();

    delay.delay_ms(1000u16);

    display.enable();
    display.clear();
    display.flush_buffer();

    //asm::bkpt();

    display.draw(Line::new((0, 0), (65, 65), 1).into_iter());
    display.draw(Font6x8::render_str("Hello World!").translate((5, 50)).into_iter());

    display.flush_buffer();
    
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
        (116, 35), (119, 41), (122, 47), (123, 53), (124, 59)];

    let mut i = 0;
    let mut pulse = false;
    loop {
        if pulse {
            extcomin.set_high();
            display.clear();
            display.draw(Line::new((65, 65), values[i], 1).into_iter());
            display.flush_buffer();

            i = (i + 1) % (values.len() -1 );
        } else {
            extcomin.set_low();
        }
        pulse = !pulse;

        delay.delay_ms(500u16);
    }

    //asm::bkpt();
}

#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    loop {}
}

