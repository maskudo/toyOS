#![no_std]
// remove main function since we dont have a runtime that calls the main function
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(toyos::test_runner)]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use toyos::{
    hlt_loop,
    memory::{self},
    println,
};
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

    // we know that there is level 1 page for address 0
    // as bootloader uses the first 1 megabyte of memory to init
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();

    hlt_loop();
}
