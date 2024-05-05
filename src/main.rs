#![no_std]
// remove main function since we dont have a runtime that calls the main function
#![no_main]

use core::panic::PanicInfo;

// function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
