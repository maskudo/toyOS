#![no_std]
// remove main function since we dont have a runtime that calls the main function
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(toyos::test_runner)]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use toyos::{allocator, hlt_loop, memory, println};
use x86_64::{
    structures::paging::{Page, Translate},
    VirtAddr,
};

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

entry_point!(kernel_main);

// entry point that is called by the bootloader. aka _start
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    toyos::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");

    println!("allocating");
    let _ = Box::new(41);
    let mut v = Vec::new();
    for i in 1..500 {
        v.push(i);
    }

    println!("vec at {:p}", v.as_slice());

    #[cfg(test)]
    test_main();
    println!("It did not crash");
    hlt_loop();
}
