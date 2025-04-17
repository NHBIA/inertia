//! Raspberry Pi Pico W 2 - Embassy UDP Example

#![no_std]
#![no_main]

use core::fmt::Write;
use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};
use defmt::*;
use embassy_executor::Spawner;
use embassy_net::udp::UdpSocket;
use embassy_net::{Config, Stack, StackResources};
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::{gpio, block::ImageDef};
use embassy_time::{Duration, Timer};
use gpio::{Level, Output};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

#[link_section = ".bi_entries"]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"Embassy UDP Example"),
    embassy_rp::binary_info::rp_program_description!(c"UDP example with Embassy and Pico W"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

static STATE: StaticCell<cyw43::State> = StaticCell::new();
static RESOURCES: StaticCell<StackResources<1>> = StaticCell::new();
static NET_STACK: StaticCell<Stack<cyw43::NetDriver<'static>>> = StaticCell::new();

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<cyw43::NetDriver<'static>>) {
    stack.run().await;
}

#[embassy_executor::task]
async fn cyw43_task(runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let fw = include_bytes!("../../firmware/cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../../firmware/cyw43-firmware/43439A0_clm.bin");

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        RM2_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );

    let state = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    unwrap!(spawner.spawn(cyw43_task(runner)));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    control
        .join_wpa2("SSID", "PASSWORD")
        .await
        .expect("Failed to connect to WiFi");

    let config = Config::dhcpv4(Default::default());
    let resources = RESOURCES.init(StackResources::new());
    let seed = embassy_net::seed::from_high_res_clock();
    let stack = NET_STACK.init(Stack::new(net_device, config, resources, seed));

    unwrap!(spawner.spawn(net_task(stack)));

    // Wait for network
    loop {
        if stack.is_link_up() {
            if let Some(cfg) = stack.config_v4() {
                info!("Got IP: {:?}", cfg.address);
                break;
            }
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    // UDP socket usage
    let mut rx_buf = [0u8; 512];
    let mut tx_buf = [0u8; 512];
    let mut socket = UdpSocket::new(stack, &mut rx_buf, &mut tx_buf);

    let message = b"Hello, UDP from Pico W!";
    let target_ip = embassy_net::Ipv4Address::new(192, 168, 1, 100);
    let target_port = 1337;

    match socket.send_to(message, (target_ip, target_port)).await {
        Ok(n) => info!("Sent {} bytes to {}:{}", n, target_ip, target_port),
        Err(e) => warn!("UDP send error: {:?}", e),
    }

    loop {
        Timer::after(Duration::from_secs(5)).await;
    }
}