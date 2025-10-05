#![no_std]
#![no_main]

mod fmt;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::adc::Adc;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Duration, Timer};
use fmt::info;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut adc = Adc::new(p.ADC1);

    let mut pin = p.PA0;

    loop {
        let value = adc.blocking_read(&mut pin);
        info!("ADC reading: {}", value);
        Timer::after(Duration::from_millis(5)).await;
    }
}
