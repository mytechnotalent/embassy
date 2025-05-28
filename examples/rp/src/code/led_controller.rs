use cyw43::{Control, PowerManagementMode};
use embassy_time::{Duration, Timer};

pub struct LedController<'a> {
    control: Control<'a>,
    clm: &'static [u8],
}

impl<'a> LedController<'a> {
    pub fn new(control: Control<'a>, clm: &'static [u8]) -> Self {
        Self { control, clm }
    }

    pub async fn init(&mut self) {
        self.control.init(self.clm).await;
        self.control.set_power_management(PowerManagementMode::PowerSave).await;
    }

    pub async fn set(&mut self, on: bool) {
        self.control.gpio_set(0, on).await;
    }

    pub async fn blink_loop(&mut self, interval: Duration) {
        loop {
            self.set(true).await;
            Timer::after(interval).await;
            self.set(false).await;
            Timer::after(interval).await;
        }
    }
}
