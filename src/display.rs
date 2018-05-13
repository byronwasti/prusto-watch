use hal;
use hal::spi::Spi;
use hal::gpio::{gpiob, gpioa, Output, PushPull, AF5};
use ls010b7dh01::Ls010b7dh01;

// Type aliases for these gross types
pub type Extcomin = gpiob::PB1<Output<PushPull>>;

pub type Display = Ls010b7dh01<
    Spi<
        hal::stm32f30x::SPI1,
        (gpioa::PA5<AF5>, gpioa::PA6<AF5>, gpioa::PA7<AF5>),
    >,
    gpiob::PB0<Output<PushPull>>,
    gpiob::PB2<Output<PushPull>>,
>;
