#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(toyos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use alloc::{boxed::Box, vec::Vec};
use bootloader::{entry_point, BootInfo};
use toyos::{
    allocator::{self, HEAP_SIZE},
    memory::{self, BootInfoFrameAllocator},
};
use x86_64::VirtAddr;

extern crate alloc;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    toyos::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");

    test_main();

    loop {}
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    toyos::test_panic_handler(panic_info)
}

#[test_case]
fn simple_allocation() {
    let x = Box::new(51);
    let y = Box::new(42);

    assert_eq!(*x, 51);
    assert_eq!(*y, 42);
}

#[test_case]
fn large_vec() {
    let n = 100;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }

    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}
