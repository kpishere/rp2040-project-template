//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

const ADS_ADDR: u8 = 0x48;

// The macro for our start-up function
use rp_pico::entry;

// defmt debugging
use defmt_rtt as _;

// Time handling traits:
use fugit::RateExtU32;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

// Clock frequency
use hal::Clock;

// Delay
use cortex_m::delay::Delay;

// External device
use hal::i2c::I2C;
use rp2040_ads::Ads1x15;
use rp2040_ads::Channel;

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
    let sda_pin: hal::gpio::Pin<_, hal::gpio::FunctionI2C, _> = pins.gpio6.reconfigure();
    let scl_pin: hal::gpio::Pin<_, hal::gpio::FunctionI2C, _> = pins.gpio7.reconfigure();

    // Create the I²C driver, using the two pre-configured pins. This will fail
    // at compile time if the pins are in the wrong mode, or if this I²C
    // peripheral isn't available on these pins!
    let i2c = hal::I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        &clocks.peripheral_clock,
    );

    let mut ads: Ads1x15<I2C<pac::I2C1, (hal::gpio::Pin<_, _, _>, hal::gpio::Pin<_, _, _>)>> =
        Ads1x15::new_ads1015(i2c);

    loop {
        let measurement = ads.read_single_ended(&mut delay, ADS_ADDR, Channel::A0);

        match measurement {
            Ok(f) => defmt::println!("{:?}", f),

            Err(_) => defmt::println!("Err"),
        }
        delay.delay_ms(1_000);
    }
}

// End of file
