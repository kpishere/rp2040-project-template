#![no_std]

//! I2C driver for the Texas Instruments ADS1015/ADS1115 ADC.
//!
//! Technical specifications:
//!
//!   - <http://www.ti.com/lit/ds/symlink/ads1015.pdf>
//!   - <http://www.ti.com/lit/ds/symlink/ads1115.pdf>
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]

use core::{
    clone::Clone, fmt, fmt::Debug, fmt::Error, marker::Copy, prelude::rust_2024::derive,
    result::Result, time::Duration,
};
use cortex_m::delay::Delay;

pub mod reg;

/// An interface to an ADS1x15 device that can be used to control the device over I2C.
pub struct Ads1x15<I2C> {
    device: I2C,
    gain: Gain,
    model: Model,
}

#[derive(Clone, Copy, Debug)]
enum Model {
    ADS1015,
    ADS1115,
}

/// A channel on the ADS1x15 that contains an analog electric signal.
#[derive(Clone, Copy, Debug)]
pub enum Channel {
    /// The channel corresponding to the `A0` pin.
    A0,
    /// The channel corresponding to the `A1` pin.
    A1,
    /// The channel corresponding to the `A2` pin.
    A2,
    /// The channel corresponding to the `A3` pin.
    A3,
}

/// Configuration for the gain setting of the device.
///
/// The gain setting sets the measurable range but it is not possible to measure voltages higher
/// than the voltage of the VDD pin of the chip.
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum Gain {
    /// The measurable range is ±6.144V.
    Within6_144V,
    /// The measurable range is ±4.096V.
    Within4_096V,
    /// The measurable range is ±2.048V.
    Within2_048V,
    /// The measurable range is ±1.024V.
    Within1_024V,
    /// The measurable range is ±0.512V.
    Within0_512V,
    /// The measurable range is ±0.256V.
    Within0_256V,
}
impl<I2C> Debug for Ads1x15<I2C> {
    /// An un-implemented implementation of Debug trait
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            _ => write!(f, "stop"),
        }
    }
}
impl<I2C> Ads1x15<I2C> {
    /// Create a new interface to an ADS1015 device.
    ///
    /// Uses the supplied I2C device.
    pub fn new_ads1015(device: I2C) -> Self {
        let gain = Gain::Within4_096V;
        let model = Model::ADS1015;

        Ads1x15 {
            device,
            gain,
            model,
        }
    }

    /// Create a new interface to an ADS1115 device.
    ///
    /// Uses the supplied I2C device.
    pub fn new_ads1115(device: I2C) -> Self {
        let gain = Gain::Within4_096V;
        let model = Model::ADS1115;

        Ads1x15 {
            device,
            gain,
            model,
        }
    }

    /// Returns the current gain setting of the device.
    pub fn gain(&self) -> Gain {
        self.gain
    }

    /// Changes the gain setting of the device.
    pub fn set_gain(&mut self, gain: Gain) {
        self.gain = gain;
    }
}

impl<I2C> Ads1x15<I2C>
where
    I2C: embedded_hal::i2c::I2c + 'static,
{
    /// Perform a device read, specify addres, channel and return a converted value.
    pub fn read_single_ended(
        &mut self,
        delay: &mut Delay,
        addr: u8,
        channel: Channel,
    ) -> Result<f32, Error> {
        use byteorder::ByteOrder;

        //        let mut device = await!(device.lock()).map_err(error::Error::Canceled::<D>)?;

        let config = reg::Config::new()
            .with_os(reg::ConfigOs::Single)
            .with_mux(channel.as_reg_config_mux_single())
            .with_pga(self.gain.as_reg_config_pga())
            .with_mode(reg::ConfigMode::Single)
            .with_dr(reg::ConfigDr::_3300SPS)
            .with_cmode(reg::ConfigCmode::Trad)
            .with_cpol(reg::ConfigCpol::Actvlow)
            .with_clat(reg::ConfigClat::Nonlat)
            .with_cque(reg::ConfigCque::None);
        let mut write_buf = [reg::Register::Config as u8, 0u8, 0u8];
        byteorder::LittleEndian::write_u16(&mut write_buf[1..], config.into());

        // Write configuration reg and trigger conversion
        self.device.write(addr, &write_buf).unwrap();
        //            .map_err(Error::InvalidWriteBufferLength)?;

        delay.delay_ms(self.model.conversion_delay().subsec_millis());
        //map_err(error::Error::Timer::<D>)?;

        // Set reg pointer to conversion
        let ptr_conversion = [0x00];
        self.device.write(addr, &ptr_conversion).unwrap();

        let mut read_buf = [0u8, 0u8];
        self.device.read(addr, &mut read_buf).unwrap();
        //            .map_err(Error::InvalidReadBufferLength)?;

        let value = self
            .model
            .convert_raw_voltage(self.gain, byteorder::BigEndian::read_i16(&read_buf));

        Ok(value)
    }
}

impl Channel {
    /// Converts this channel value into a valid value for the I2C `Config` register, setting the
    /// mux to single-ended measurements for that channel.
    pub fn as_reg_config_mux_single(&self) -> reg::ConfigMux {
        match *self {
            Channel::A0 => reg::ConfigMux::Single0,
            Channel::A1 => reg::ConfigMux::Single1,
            Channel::A2 => reg::ConfigMux::Single2,
            Channel::A3 => reg::ConfigMux::Single3,
        }
    }
}

impl Gain {
    /// Converts this gain value into a valid value for the I2C `Config` register.
    pub fn as_reg_config_pga(&self) -> reg::ConfigPga {
        match *self {
            Gain::Within6_144V => reg::ConfigPga::_6_144V,
            Gain::Within4_096V => reg::ConfigPga::_4_096V,
            Gain::Within2_048V => reg::ConfigPga::_2_048V,
            Gain::Within1_024V => reg::ConfigPga::_1_024V,
            Gain::Within0_512V => reg::ConfigPga::_0_512V,
            Gain::Within0_256V => reg::ConfigPga::_0_256V,
        }
    }
}

impl Model {
    fn conversion_delay(&self) -> Duration {
        match *self {
            Model::ADS1015 => Duration::from_millis(1),
            Model::ADS1115 => Duration::from_millis(8),
        }
    }

    fn convert_raw_voltage(&self, gain: Gain, value: i16) -> f32 {
        match *self {
            Model::ADS1015 => {
                let value = (value >> 4) as f32;
                match gain {
                    Gain::Within6_144V => value * 3.0000e-3,
                    Gain::Within4_096V => value * 2.0000e-3,
                    Gain::Within2_048V => value * 1.0000e-3,
                    Gain::Within1_024V => value * 5.0000e-4,
                    Gain::Within0_512V => value * 2.5000e-4,
                    Gain::Within0_256V => value * 1.2500e-4,
                }
            }
            Model::ADS1115 => {
                let value = value as f32;
                match gain {
                    Gain::Within6_144V => value * 1.8750e-4,
                    Gain::Within4_096V => value * 1.2500e-4,
                    Gain::Within2_048V => value * 6.2500e-5,
                    Gain::Within1_024V => value * 3.1250e-5,
                    Gain::Within0_512V => value * 1.5625e-5,
                    Gain::Within0_256V => value * 7.8125e-6,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    const ADS_ADDR: u8 = 0x48;

    // For string formatting.
    use core::fmt::Write;

    // The macro for our start-up function
    use rp_pico::entry;

    // Time handling traits:
    use fugit::RateExtU32;

    // Timer for the delay on the display:
    use embedded_hal::delay::DelayNs;

    // Ensure we halt the program on panic (if we don't mention this crate it won't
    // be linked)
    use panic_halt as _;

    // A shorter alias for the Peripheral Access Crate, which provides low-level
    // register access
    use rp_pico::hal::pac;

    // A shorter alias for the Hardware Abstraction Layer, which provides
    // higher-level drivers.
    use rp_pico::hal;

    use crate::hal::I2C;

    use crate::Ads1x15;

    // The library driver:
    use super::*;

    /// Entry point to our bare-metal application.
    ///
    /// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
    /// as soon as all global variables are initialised.
    ///
    /// The function configures the RP2040 peripherals,
    /// gets a handle on the I2C peripheral,
    /// initializes the SSD1306 driver, initializes the text builder
    /// and then draws some text on the display.
    #[test]
    fn main() {
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
        let i2c = i2c1(
            pac.I2C1,
            sda_pin,
            scl_pin,
            400.kHz(),
            &mut pac.RESETS,
            &clocks.peripheral_clock,
        );

        let mut ads: Ads1x15<I2C<pac::I2C1, (hal::gpio::Pin<_, _, _>, hal::gpio::Pin<_, _, _>)>> =
            Ads1x15::new_ads1015(i2c);

        let count = 10;
        while count {
            let measurement = ads.read_single_ended(&mut delay, ADS_ADDR, Channel::A0);

            match measurement {
                Ok(f) => defmt::println!("{:?}", f),

                Err(e) => defmt::println!("Err: {:?}", e),
            }
            delay.delay_ms(1_000);
        }
    }
}
