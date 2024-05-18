#![no_std]
// remove main function since we dont have a runtime that calls the main function
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(toyos::test_runner)]

use core::panic::PanicInfo;
use toyos::{
    println,
    vga_buffer::{BUFFER_HEIGHT, WRITER},
};

// function called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
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

#[test_case]
fn test_println_simple() {
    println!("test println simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test println many outputs");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        // since println puts a newline after the string, the string is present in the
        // BUFFER_HEIGHT - 2th line
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello wolr{}", "d");

    #[cfg(test)]
    test_main();

    loop {}
}
