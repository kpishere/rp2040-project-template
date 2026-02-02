//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
//! ref: https://pico.implrust.com/spi/spi-in-raspberry-pi-pico-2.html
//!
#![no_std]
#![no_main]

use cortex_m::prelude::_embedded_hal_blocking_spi_Write;
// The macro for our start-up function
use rp_pico::entry;

// defmt debugging
use defmt::*;
use defmt_rtt as _;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Timer for the delay on the display:
use embedded_hal::{delay::DelayNs, digital::OutputPin};

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
// Pull in any important traits
// Import the SPI abstraction:
// Import the GPIO abstraction:

use rp_pico::hal::{self, Clock, gpio, pac, prelude::*, spi};

// Time handling traits:
use fugit::RateExtU32;

// Delay
use cortex_m::delay::Delay;

use core::{
    clone::Clone, fmt, fmt::Debug, fmt::Error, marker::Copy, prelude::rust_2024::derive,
    result::Result, time::Duration,
};

// External device
use embedded_graphics::prelude::*;
use embedded_graphics::text::Baseline;
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    text::Text,
};
//use embedded_hal::digital::OutputPin;
//use embedded_hal::digital::v2::OutputPin;
//use embedded_time::{fixed_point::FixedPoint, rate::Extensions};

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals,
/// gets a handle on the I2C peripheral,
/// initializes the SSD1306 driver, initializes the text builder
/// and then draws some text on the display.

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Device driver wants to delay in write/read cycle
    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Configure two pins as being I²C, not GPIO
    let spi_clk = pins.gpio10.into_function::<hal::gpio::FunctionSpi>();
    let spi_tx = pins.gpio11.into_function::<hal::gpio::FunctionSpi>();
    let mut led_latch = pins.gpio15.into_push_pull_output();
    let mut spi_bus = hal::spi::Spi::<_, _, _, 8>::new(pac.SPI1, (spi_tx, spi_clk)).init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        16.MHz(),
        embedded_hal::spi::MODE_0,
    );

    let mut data = [0x80u8, 0x00u8, 0x80u8];

    loop {
        data[0] = data[0].rotate_left(1);
        data[2] = data[2].rotate_left(3);
        defmt::println!("{:?}", &data);

        led_latch.set_high().unwrap();
        let _ = spi_bus.write(&data).is_ok();
        led_latch.set_low().unwrap();

        delay.delay_ms(1_000);
    }
}

// End of file
