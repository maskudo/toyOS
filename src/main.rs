#![no_std]
// remove main function since we dont have a runtime that calls the main function
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// function called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello wolr{}", "d");
    panic!("panic lol");
    loop {}
}
