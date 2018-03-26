#![no_std]

extern crate cortex_m;
extern crate stm32f30x_hal as hal;
extern crate embedded_hal;
extern crate ls010b7dh01;
#[macro_use(block)]
extern crate nb;

// TODO Remove this dependancy
use embedded_hal::spi::{Mode, Phase, Polarity};

use cortex_m::asm;
use hal::prelude::*;
use hal::spi::Spi;
use hal::stm32f30x;
use hal::delay::Delay;

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
        .pb14
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper); // DISP_EN

    disp_en.set_high();

    // Set up our CS (Active high)
    let mut cs = gpiob
        .pb2
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper); // CS
    cs.set_low();

    // Set up 5V_en
    let mut v5_en = gpioa
        .pa1
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper); // 5V_en
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

    //let mode = Mode { polarity: Polarity::IdleLow, phase: Phase::CaptureOnFirstTransition };
    let mode = Mode { polarity: Polarity::IdleLow, phase: Phase::CaptureOnSecondTransition };
    let mut spi = Spi::spi1(p.SPI1, (pa5, pa6, pa7), mode, 1.mhz(), clocks, &mut rcc.apb2);

    // Wait to let everything set up
    delay.delay_ms(200_u16);

    asm::bkpt();

    //let clear_data = [0b0010_0000, 0x00];
    let clear_data = [0x20, 0x00];
    let set_line_value = 
        [ 0x80, 0xba,
        0x33, 0x33, 0x33, 0x33,
        0x33, 0x33, 0x33, 0x33,
        0x33, 0x33, 0x33, 0x33,
        0x33, 0x33, 0x33, 0x33,
        0x00, 0x00 ];
    let set_display = [0x00, 0x00];

    // Send data
    cs.set_high();
    spi.write(&clear_data);
    //delay.delay_ms(5_u16);
    cs.set_low();

    asm::bkpt();

    // Send data
    cs.set_high();
    spi.write(&set_line_value);
    //delay.delay_ms(5_u16);
    cs.set_low();

    asm::bkpt();

    // Send data
    cs.set_high();
    spi.write(&set_display);
    //delay.delay_ms(5_u16);
    cs.set_low();

    asm::bkpt()
}
