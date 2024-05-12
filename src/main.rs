#![no_std]
// remove main function since we dont have a runtime that calls the main function
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

use crate::vga_buffer::{BUFFER_HEIGHT, WRITER};
use core::panic::PanicInfo;

mod serial;
mod vga_buffer;

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
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    // we use 0x10 and 0x11 as opposed to 0x0 and 0x1 which are the standard exit codes for programs
    // because qemu generates exit code of our program as eqn: (OUR_CODE << 1) | 1
    // which results in a qemu exit code of 1 if we used the standard exit code of 0 to represent
    // our teest pass
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) -> () {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
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
