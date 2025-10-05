#![no_std]
#![no_main]

mod fmt;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use analogue_embassy::is_position_max;
use embassy_executor::Spawner;
use embassy_stm32::adc::Adc;
use embassy_time::{Duration, Timer};
use fmt::info;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut adc = Adc::new(p.ADC1);

    let mut pin = p.PA0;

    loop {
        let value = is_position_max(&mut adc, &mut pin);
        info!("is position max? {}", value);
        Timer::after(Duration::from_millis(5)).await;
    }
}
