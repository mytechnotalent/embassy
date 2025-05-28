use cyw43_pio::{PioSpi, DEFAULT_CLOCK_DIVIDER};
use embassy_rp::gpio::{Level, Output};
use embassy_rp::{
    bind_interrupts,
    peripherals::{DMA_CH0, PIO0},
    pio::{InterruptHandler, Pio},
};
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

pub type Cyw43Runner = cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>;
pub type Cyw43Control = cyw43::Control<'static>;

static STATE: StaticCell<cyw43::State> = StaticCell::new();

pub async fn init_wifi() -> (Cyw43Control, Cyw43Runner, &'static [u8]) {
    let p = embassy_rp::init(Default::default());
    let fw = include_bytes!("../../../../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../../../../cyw43-firmware/43439A0_clm.bin");

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );

    let state = STATE.init(cyw43::State::new());
    let (_net_device, control, runner) = cyw43::new(state, pwr, spi, fw).await;
    (control, runner, clm)
}
