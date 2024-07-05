#![no_std]
// remove main function since we dont have a runtime that calls the main function
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(toyos::test_runner)]

use core::panic::PanicInfo;
use toyos::{hlt_loop, println};

// function called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    toyos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello wolr{}", "d");

    toyos::init();

    #[cfg(test)]
    test_main();
    println!("didnt crash");

    hlt_loop();
}
