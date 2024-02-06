//! Blinks the LED on a Pico board

#![no_std]
#![no_main]

use core::arch::asm;

use bsp::entry;
use cortex_m::peripheral::{syst::SystClkSource, SYST};
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
use rp_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use symex_lib::{end_cyclecount, any, start_cyclecount, symbolic, assume, Any, black_box, suppress_path, Valid};

#[entry]
fn main() -> ! {
    info!("Ex1 start");
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

    let systic_reload_time: u32 = 0x00ffffff;
    let mut systic = core.SYST;
    systic.set_clock_source(SystClkSource::Core);
    systic.set_reload(systic_reload_time);
    systic.enable_counter();

    //measure_hw();
    //small_timing_test();
    //smaller_timing_test();
    //measure_symex();
    let r = test_any();
    info!("r: {}", r);
    let r = test_validate();
    info!("r: {}", r);
    loop {}
}

enum Inner {
    One,
    Two(u16, u16),
}

impl Any for Inner {
    fn any() -> Self {
        match u8::any() {
            0 => Inner::One,
            _ => Inner::Two(u16::any(), u16::any()),
        }
    }
}

enum TestEnum {
    One,
    Two,
    Three(u16),
    Four(Inner),
    Five(u16),
}

impl Any for TestEnum {
    fn any() -> Self {
        let mut n = 1u8;
        symbolic(&mut n);
        match n {
            0 => TestEnum::One,
            1 => TestEnum::Two,
            2 => TestEnum::Three(u16::any()),
            3 => TestEnum::Four(Inner::any()),
            _ => TestEnum::Five(u16::any()),
        }
    }
}

enum TestEnum2 {
    One,
    Two,
    Three(u16),
    Four(u16, u16),
    Five(u16),
}

#[inline(never)]
#[no_mangle]
fn enum_2_if(input: &TestEnum2) {
    let mut ret = 0;

    let input = unsafe {
        let raw_pointer = core::ptr::addr_of!(*input);
        core::ptr::read_volatile(raw_pointer as *const TestEnum2)
    };

    if let TestEnum2::One = input {
        ret = 1;
    } else if let TestEnum2::Two = input {
        ret = 2;
    } else if let TestEnum2::Three(_) = input {
        ret = 3;
    } else if let TestEnum2::Four(_, _) = input {
        ret = 4;
    } else if let TestEnum2::Five(_) = input {
        ret = 5;
    } else {
        suppress_path()
    }
    black_box(&mut ret);
}

impl Valid for TestEnum2 {

    #[inline(never)]
    #[no_mangle]
    fn is_valid(&self) -> bool {
        let mut ret = false;

        let input = unsafe {
            let raw_pointer = core::ptr::addr_of!(*self);
            core::ptr::read_volatile(raw_pointer as *const TestEnum2)
        };

        if let TestEnum2::One = input {
            ret = true;
        } else if let TestEnum2::Two = input {
            ret = true;
        } else if let TestEnum2::Three(_) = input {
            ret = true;
        } else if let TestEnum2::Four(_, _) = input {
            ret = true;
        } else if let TestEnum2::Five(_) = input {
            ret = true;
        } else {
            suppress_path()
        }
        black_box(&mut ret);
        ret
    }
}



#[inline(never)]
#[no_mangle]
fn test_any() -> u16 {
    let input: TestEnum = any();
    let r = handle_test_enum(input);
    r
}

#[inline(never)]
#[no_mangle]
fn test_validate() -> u16 {
    let mut input: TestEnum2 = TestEnum2::One;
    symbolic(&mut input);
    //enum_2_if(&input);
    input.is_valid();
    let r = handle_test_enum2(input);
    r
}

#[inline(never)]
#[no_mangle]
fn handle_test_enum2(n: TestEnum2) -> u16 {
    match n {
        TestEnum2::One => 1,
        TestEnum2::Two => simple_if(2),
        TestEnum2::Three(v) => v,
        TestEnum2::Four(i1, i2) => i1 + i2,
        TestEnum2::Five(v) => simple_if(v),
    }
}


#[inline(never)]
#[no_mangle]
fn handle_test_enum(n: TestEnum) -> u16 {
    match n {
        TestEnum::One => 1,
        TestEnum::Two => simple_if(2),
        TestEnum::Three(v) => v,
        TestEnum::Four(i) => {match i {
            Inner::One => 1,
            Inner::Two(a, b) => a + b,
        }},
        TestEnum::Five(v) => simple_if(v),
    }
}


#[inline(never)]
#[no_mangle]
fn simple_if(n: u16) -> u16 {
    if n == 3 {
        1
    } else if n == 6 {
        5
    } else {
        2
    }
}
