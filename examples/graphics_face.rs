#![feature(used)]
#![no_std]
extern crate cortex_m;
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

use graphics::prelude::*;
use graphics::Drawing;
use graphics::primitives::{Circle, Line, Rect};
use graphics::fonts::{Font, Font6x8};
use graphics::transform::Transform;
use graphics::image::Image1BPP;

fn main() {
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
    let mut display = Ls010b7dh01::new(spi, cs, disp_en);

    display.disable();
    delay.delay_ms(200u16);
    display.enable();
    display.clear();
    display.flush_buffer();

    let bpp = Image1BPP::new(include_bytes!("../data/face_1bpp_neg.raw"), 120, 120).translate((0, 0));
    display.draw(bpp.into_iter());

    display.flush_buffer();
    
    let mut i = 0;
    let mut pulse = false;
    loop {
        if pulse {
            extcomin.set_high();
        } else {
            extcomin.set_low();
        }
        pulse = !pulse;

        delay.delay_ms(500u16);
    }
}

