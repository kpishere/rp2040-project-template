//! Definitions for ADS1x15 I2C registers.
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(missing_debug_implementations)]
#![allow(missing_copy_implementations)]

/*use core::{
    clone::Clone, cmp::Eq, cmp::Ord, cmp::PartialEq, cmp::PartialOrd, convert::From, convert::Into,
    fmt::Debug, marker::Copy, prelude::rust_2024::derive, todo,
};
*/
//use core::prelude::rust_2024::derive;
use bitfield_struct::{bitenum, bitfield};

/// I2C registers present in an ADS1x15.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Register {
    /// The `Convert` register.
    Convert = 0x00,
    /// The `Config` register.
    Config = 0x01,
    /// The `Lowthresh` register.
    Lowthresh = 0x02,
    /// The `Hithresh` register.
    Hithresh = 0x03,
}

/// Valid values for the `Config` register.
#[bitfield(u16)]
pub struct Config {
    #[bits(2)]
    pub cque: ConfigCque,
    #[bits(1)]
    pub clat: ConfigClat,
    #[bits(1)]
    pub cpol: ConfigCpol,
    #[bits(1)]
    pub cmode: ConfigCmode,
    #[bits(3)]
    pub dr: ConfigDr,
    #[bits(1)]
    pub mode: ConfigMode,
    #[bits(3)]
    pub pga: ConfigPga,
    #[bits(3)]
    pub mux: ConfigMux,
    #[bits(1)]
    pub os: ConfigOs,
}
/*
impl Into<u16> for Config {
    fn into(self) -> u16 {
        (self.0[0] as u16) << 8 | self.0[1] as u16
    }
}

impl From<u16> for Config {
    fn from(value: u16) -> Self {
        Config([((value >> 8) & 0xff) as u8, (value & 0xff) as u8])
    }
}
*/

/// Values for the `Os` part of the `Config` register.
/// Example: assert_eq!(ConfigOs::from_bits(0), ConfigOs::Noop);
///          assert_eq!(ConfigOs::Single.into_bits(), 1);
#[bitenum]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConfigOs {
    /// `Os`: Write: Noop.
    #[fallback]
    Noop = 0b0,
    /// `Os`: Write: Set to start a single-conversion.
    Single = 0b1,
}

/// Values for the `Mux` part of the `Config` register.
#[bitenum]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConfigMux {
    /// `Mux`: Differential P = AIN0, N = AIN1 (default).
    #[fallback]
    Diff0_1 = 0b000,
    /// `Mux`: Differential P = AIN0, N = AIN3.
    Diff0_3 = 0b001,
    /// `Mux`: Differential P = AIN1, N = AIN3.
    Diff1_3 = 0b010,
    /// `Mux`: Differential P = AIN2, N = AIN3.
    Diff2_3 = 0b011,
    /// `Mux`: Single-ended AIN0.
    Single0 = 0b100,
    /// `Mux`: Single-ended AIN1.
    Single1 = 0b101,
    /// `Mux`: Single-ended AIN2.
    Single2 = 0b110,
    /// `Mux`: Single-ended AIN3.
    Single3 = 0b111,
}

/// Values for the `Pga` part of the `Config` register.
#[bitenum]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConfigPga {
    /// `Pga`: +/-6.144V range = Gain 2/3.
    _6_144V = 0b000,
    /// `Pga`: +/-4.096V range = Gain 1.
    _4_096V = 0b001,
    /// `Pga`: +/-2.048V range = Gain 2 (default).
    #[fallback]
    _2_048V = 0b010,
    /// `Pga`: +/-1.024V range = Gain 4.
    _1_024V = 0b011,
    /// `Pga`: +/-0.512V range = Gain 8.
    _0_512V = 0b100,
    /// `Pga`: +/-0.256V range = Gain 16.
    _0_256V = 0b101,
    // unused: 0b110,
    // unused: 0b111,
}

/// Values for the `Mode` part of the `Config` register.
#[bitenum]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConfigMode {
    /// `Mode`: Continuous conversion mode.
    Contin = 0b0,
    /// `Mode`: Power-down single-shot mode (default).
    #[fallback]
    Single = 0b1,
}

/// Values for the `Dr` part of the `Config` register.
#[bitenum]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConfigDr {
    /// `Dr`: 128 samples per second.
    _128SPS = 0b000,
    /// `Dr`: 250 samples per second.
    _250SPS = 0b001,
    /// `Dr`: 490 samples per second.
    _490SPS = 0b010,
    /// `Dr`: 920 samples per second.
    _920SPS = 0b011,
    /// `Dr`: 1600 samples per second (default).
    #[fallback]
    _1600SPS = 0b100,
    /// `Dr`: 2400 samples per second.
    _2400SPS = 0b101,
    /// `Dr`: 3300 samples per second.
    _3300SPS = 0b110,
    // unused: 0b111,
}

/// Values for the `Cmode` part of the `Config` register.
#[bitenum]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConfigCmode {
    /// `Cmode`: Traditional comparator with hysteresis (default).
    #[fallback]
    Trad = 0b0,
    /// `Cmode`: Window comparator.
    Window = 0b1,
}

/// Values for the `Cpol` part of the `Config` register.
#[bitenum]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConfigCpol {
    /// `Cpol`: ALERT/RDY pin is low when active (default).
    #[fallback]
    Actvlow = 0b0,
    /// `Cpol`: ALERT/RDY pin is high when active.
    Actvhi = 0b1,
}

/// Values for the `Clat` part of the `Config` register.
#[bitenum]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConfigClat {
    /// `Clat`: Non-latching comparator (default).
    #[fallback]
    Nonlat = 0b0,
    /// `Clat`: Latching comparator.
    Latch = 0b1,
}

/// Values for the `Cque` part of the `Config` register.
#[bitenum]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConfigCque {
    /// `Cque`: Assert ALERT/RDY after one conversions.
    Conv1 = 0b00,
    /// `Cque`: Assert ALERT/RDY after two conversions.
    Conv2 = 0b01,
    /// `Cque`: Assert ALERT/RDY after four conversions.
    Conv4 = 0b10,
    /// `Cque`: Disable the comparator and put ALERT/RDY in high state (default).
    #[fallback]
    None = 0b11,
}
