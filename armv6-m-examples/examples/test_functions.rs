//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

// Add the bootloader
#[link_section = ".boot2"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;
//use panic_halt as _;

use rp2040_boot2;
use rp2040_hal as hal;
use symex_lib::{any, assume};

use hal::{
    clocks::{init_clocks_and_plls, Clock},
    entry, pac,
    sio::Sio,
    watchdog::Watchdog,
};

#[inline(never)]
#[no_mangle]
fn panic_test_defmt(n: u8) -> u8 {
    if n < 8 {
        test_simple_if(n)
    } else {
        defmt::panic!()
    }
}

#[inline(never)]
#[no_mangle]
fn panic_test_core(n: u8) -> u8 {
    if n < 8 {
        test_simple_if(n)
    } else {
        core::panic!()
    }
}

#[inline(never)]
#[no_mangle]
fn panic_test_divide_by_zero(n: u8) -> u8 {
    if n < 8 {
        16 / (test_simple_if(n) - 1)
    } else {
        core::panic!()
    }
}

#[inline(never)]
#[no_mangle]
fn simple_loop(n: u8) -> u8 {
    let mut sum = 0;
    for i in 0..n {
        sum += test_simple_if(i);
    }
    sum
}

#[inline(never)]
#[no_mangle]
fn test_nested_if_over_function(n: u8) -> u8 {
    if n > 5 {
        99
    } else {
        test_simple_if(n)
    }
}

#[inline(never)]
#[no_mangle]
fn test_simple_if(n: u8) -> u8 {
    if n == 3 {
        1
    } else if n == 6 {
        5
    } else {
        2
    }
}

#[inline(never)]
fn simple_loop_llvm() {
    let n = any();
    simple_loop(n);
}


fn run_function(f: fn(u8) -> u8) {
    for n in 0..30 {
        let r = f(n);
        info!("{} {}", n, r);
    }
}

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

    run_function(test_simple_if);
    run_function(test_nested_if_over_function);
    run_function(simple_loop);
    run_function(panic_test_divide_by_zero);
    run_function(panic_test_core);
    run_function(panic_test_defmt);
    simple_loop_llvm();
    loop {}

}

// End of file
