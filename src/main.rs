//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(allocator_api)]

extern crate alloc;

use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
use core::mem::MaybeUninit;
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
//use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use rp2040_hal::{clocks::init_clocks_and_plls, clocks::Clock, pac, sio::Sio, watchdog::Watchdog};

// Specify boot block at start of image for linker
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
const HEAP_SIZE: usize = 1024 * 16;
static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

// Green threads
use corosensei::stack::MIN_STACK_SIZE;
use corosensei::{Coroutine, CoroutineResult};

mod pico;

#[entry]
fn main() -> ! {
    info!("Program setup");
    unsafe { ALLOCATOR.init(HEAP.as_ptr() as usize, HEAP_SIZE) }

    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let _clocks = init_clocks_and_plls(
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

    let mut delay = cortex_m::delay::Delay::new(core.SYST, _clocks.system_clock.freq().integer());

    let pins = rp2040_hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.gpio25.into_push_pull_output();

    let stack: pico::DefaultStack = pico::DefaultStack::new(MIN_STACK_SIZE).unwrap();
    let mut coroutine = Coroutine::with_stack(stack, |yielder, input| {
        info!("[coroutine] coroutine started with input {}", input);
        for i in 0..5 {
            info!("[coroutine] yielding {}", i);
            let input: i32 = yielder.suspend(i);
            info!("[coroutine] got {} from parent", input)
        }
        info!("[coroutine] exiting coroutine");
    });
    let mut counter = 100;
    let mut completed = false;

    loop {
        if !completed {
            info!("[main] resuming coroutine with argument {}", counter);
            match coroutine.resume(counter) {
                CoroutineResult::Yield(i) => info!("[main] got {:?} from coroutine", i),
                CoroutineResult::Return(()) => completed = true,
            }
            counter += 1;
        }

        info!("on!");
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        info!("off!");
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}

// End of file
