//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
//use embedded_hal::digital::OutputPin;
use embedded_hal::digital::StatefulOutputPin;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{Clock, init_clocks_and_plls},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    //let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // This is the correct pin on the Raspberry Pico board. On other boards, even if they have an
    // on-board LED, it might need to be changed.
    //
    // Notably, on the Pico W, the LED is not connected to any of the RP2040 GPIOs but to the cyw43 module instead.
    // One way to do that is by using [embassy](https://github.com/embassy-rs/embassy/blob/main/examples/rp/src/bin/wifi_blinky.rs)
    //
    // If you have a Pico W and want to toggle a LED with a simple GPIO output pin, you can connect an external
    // LED to one of the GPIO pins, and reference that pin here. Don't forget adding an appropriate resistor
    // in series with the LED.
    let mut gpio0 = pins.gpio0.into_push_pull_output();
    let mut gpio1 = pins.gpio1.into_push_pull_output();
    let mut gpio2 = pins.gpio2.into_push_pull_output();
    let mut gpio3 = pins.gpio3.into_push_pull_output();
    let mut gpio4 = pins.gpio4.into_push_pull_output();
    let mut gpio5 = pins.gpio5.into_push_pull_output();
    let mut gpio6 = pins.gpio6.into_push_pull_output();
    let mut gpio7 = pins.gpio7.into_push_pull_output();
    let mut gpio8 = pins.gpio8.into_push_pull_output();
    let mut gpio9 = pins.gpio9.into_push_pull_output();
    let mut gpio10 = pins.gpio10.into_push_pull_output();
    let mut gpio11 = pins.gpio11.into_push_pull_output();
    let mut gpio12 = pins.gpio12.into_push_pull_output();
    let mut gpio13 = pins.gpio13.into_push_pull_output();
    let mut gpio14 = pins.gpio14.into_push_pull_output();
    let mut gpio15 = pins.gpio15.into_push_pull_output();
    let mut gpio16 = pins.gpio16.into_push_pull_output();
    let mut gpio17 = pins.gpio17.into_push_pull_output();
    let mut gpio18 = pins.gpio18.into_push_pull_output();
    let mut gpio19 = pins.gpio19.into_push_pull_output();
    let mut gpio20 = pins.gpio20.into_push_pull_output();
    let mut gpio21 = pins.gpio21.into_push_pull_output();
    let mut gpio22 = pins.gpio22.into_push_pull_output();

    let mut led = pins.led.into_push_pull_output();
    let mut gpio26 = pins.gpio26.into_push_pull_output();
    let mut gpio27 = pins.gpio27.into_push_pull_output();
    let mut gpio28 = pins.gpio28.into_push_pull_output();

    let mut ptr = 0;
    loop {
        match ptr % 29 {
            0 => {
                gpio0.toggle();
            }
            1 => {
                gpio1.toggle();
            }
            2 => {
                gpio2.toggle();
            }
            3 => {
                gpio3.toggle();
            }
            4 => {
                gpio4.toggle();
            }
            5 => {
                gpio5.toggle();
            }
            6 => {
                gpio6.toggle();
            }
            7 => {
                gpio7.toggle();
            }
            8 => {
                gpio8.toggle();
            }
            9 => {
                gpio9.toggle();
            }
            10 => {
                gpio10.toggle();
            }
            11 => {
                gpio11.toggle();
            }
            12 => {
                gpio12.toggle();
            }
            13 => {
                gpio13.toggle();
            }
            14 => {
                gpio14.toggle();
            }
            15 => {
                gpio15.toggle();
            }
            16 => {
                gpio16.toggle();
            }
            17 => {
                gpio17.toggle();
            }
            18 => {
                gpio18.toggle();
            }
            19 => {
                gpio19.toggle();
            }
            20 => {
                gpio20.toggle();
            }
            21 => {
                gpio21.toggle();
            }
            22 => {
                gpio22.toggle();
            }
            23 => {
                led.toggle();
            }
            26 => {
                gpio26.toggle();
            }
            27 => {
                gpio27.toggle();
            }
            28 => {
                gpio28.toggle();
            }
            _ => (),
        }
        ptr+=1;
    }
}

// End of file
