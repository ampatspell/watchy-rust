#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::{clock::CpuClock, interrupt::software::SoftwareInterruptControl};
use esp_hal::timer::timg::TimerGroup;
use watchy::display::{DisplayPins, spawn_display_task};
use {esp_backtrace as _, esp_println as _};

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 98768);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let software_interrupt = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, software_interrupt.software_interrupt0);

    spawn_display_task(
        &spawner,
        DisplayPins {
            busy: esp_hal::gpio::Pin::degrade(peripherals.GPIO19),
            rst: esp_hal::gpio::Pin::degrade(peripherals.GPIO9),
            dc: esp_hal::gpio::Pin::degrade(peripherals.GPIO10),
            cs: esp_hal::gpio::Pin::degrade(peripherals.GPIO5),
            sck: esp_hal::gpio::Pin::degrade(peripherals.GPIO18),
            mosi: esp_hal::gpio::Pin::degrade(peripherals.GPIO23),
            spi: peripherals.SPI2,
        },
    );

    loop {
        info!("main");
        Timer::after(Duration::from_secs(5)).await;
    }
}
