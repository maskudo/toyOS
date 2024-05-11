#![no_std]
// remove main function since we dont have a runtime that calls the main function
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello world").unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", some numbers: {} {}",
        42,
        1.337
    )
    .unwrap();
    loop {}
}
