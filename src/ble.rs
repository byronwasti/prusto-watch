use hal;
use hal::serial::Serial;
use hal::gpio::{gpiob, Output, PushPull, AF7};
use rn4870;

type BleSerial = Serial<hal::stm32f30x::USART1, (gpiob::PB6<AF7>, gpiob::PB7<AF7>)>;
pub type Ble = rn4870::Rn4870<BleSerial, gpiob::PB5<Output<PushPull>>>;
