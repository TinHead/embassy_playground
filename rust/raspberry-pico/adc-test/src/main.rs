#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::adc::{Adc, Config};
use embassy_rp::interrupt;
use embassy_time::{Duration, Timer};

use {defmt_rtt as _, panic_probe as _};

fn convert_to_celsius(raw_temp: u16) -> f32 {
    // According to chapter 4.9.5. Temperature Sensor in RP2040 datasheet
    27.0 - (raw_temp as f32 * 3.3 / 4096.0 - 0.706) / 0.001721 as f32
}
#[embassy_executor::task]
async fn main(_spawner: Spawner) {
    info!("Hello, world!");
    let p = embassy_rp::init(Default::default());
    let irq = interrupt::take!(ADC_IRQ_FIFO);
    let mut adc = Adc::new(p.ADC, irq, Config::default());
    loop {
        let temp = adc.read_temperature().await;
        info!("Temp: {} degrees", convert_to_celsius(temp));
        Timer::after(Duration::from_secs(1)).await;
    }
}
