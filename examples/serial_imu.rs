#![no_std]

extern crate cortex_m;
extern crate stm32f30x_hal as hal;
extern crate rn4870;
extern crate ls010b7dh01;
#[macro_use(block)]
extern crate nb;

use cortex_m::asm;
use hal::prelude::*;
use hal::serial::Serial;
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

    // Set up serial
    let tx = gpiob
        .pb10
        .into_af7(&mut gpiob.moder, &mut gpiob.afrh);

    let rx = gpiob
        .pb11
        .into_af7(&mut gpiob.moder, &mut gpiob.afrh);

    let mut serial = Serial::usart3(p.USART3, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb1);

    // Set up a delay
    let mut delay = Delay::new(cp.SYST, clocks);

    // Set up our CS (Active high)
    let mut cs = gpioa
        .pa3
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper); // CS
    cs.set_high();

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
    let mut spi = Spi::spi1(p.SPI1, (pa5, pa6, pa7), mode, 125.khz(), clocks, &mut rcc.apb2);

    // Wait a bit then try to get whoami
    delay.delay_ms(1000u16);
    let mut buffer = [0b1000_1111, 0x00];

    cs.set_low();
    let res = spi.transfer(&mut buffer);
    cs.set_high();

    match res {
        Ok(val) => {
            asm::bkpt();
            block!(serial.write(b'r'));
            block!(serial.write(b':'));
            block!(serial.write(b' '));
            block!(serial.write(val[0]));
            block!(serial.write(val[1]));
        },
        Err(_) => panic!("Spi err"),
    }


    asm::bkpt();
    loop {
        //block!(serial.write(value));
    }

    asm::bkpt();
}
