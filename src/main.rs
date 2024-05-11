#![no_std]
// remove main function since we dont have a runtime that calls the main function
#![no_main]

use core::panic::PanicInfo;

use vga_buffer::print_something;
mod vga_buffer;

// function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unimplemented!();
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_something();

    loop {}
}
