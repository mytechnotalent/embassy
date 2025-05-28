//! This example tests the RP Pico W on-board LED.
//!
//! It does not work with the RP Pico board. See blinky.rs.

#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_time::Duration;

mod led_controller;
mod wifi;

use led_controller::LedController;
use wifi::{init_wifi, Cyw43Runner};

#[embassy_executor::task]
async fn wifi_task(runner: Cyw43Runner) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let (control, runner, clm) = init_wifi().await;
    spawner.spawn(wifi_task(runner)).unwrap();

    let mut led = LedController::new(control, clm);
    led.init().await;
    led.blink_loop(Duration::from_secs(1)).await;
}
