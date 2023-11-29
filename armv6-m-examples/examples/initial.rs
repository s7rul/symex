#![no_std]
#![no_main]
//! Simple example showcasing what symbolic execution can do.
//!
//! ```shell
//! cargo symex --example initial --function foo
//! ```
#![allow(dead_code)]
use panic_halt as _;
use rp2040_hal::entry;
use symex_lib::Any;

fn bar(x: i32, y: i32) -> i32 {
    if x > 5 && x + y == 100 {
        if x * y == 1875 {
            panic!();
        } else {
            5
        }
    } else {
        x / y
    }
}

#[inline(never)]
#[no_mangle]
fn foo() -> i32 {
    let x = i32::any();
    let y = i32::any();
    bar(x, y)
}

#[entry]
fn main() -> ! {
    let n = foo();

    unsafe {
        let _ = core::ptr::read_volatile(&n);
    }

    loop {}
}
