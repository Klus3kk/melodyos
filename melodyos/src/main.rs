#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(alloc_error_handler)]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::VirtAddr;

mod allocator;
mod memory;
mod vga_buffer;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Welcome to MelodyOS!");

    // Print memory information
    println!("Memory Information:");
    memory::print_memory_info(&boot_info.memory_map);

    // Initialize memory management
    println!("Initializing memory management...");
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    
    // Initialize memory map
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    
    // Map a test page to demonstrate our memory management capabilities
    let page = x86_64::structures::paging::Page::containing_address(
        VirtAddr::new(0x1000)
    );
    
    match memory::map_page(
        page,
        x86_64::structures::paging::PhysFrame::containing_address(
            x86_64::PhysAddr::new(0x2000)
        ),
        &mut mapper,
        &mut frame_allocator
    ) {
        Ok(_) => println!("Successfully mapped a test page!"),
        Err(e) => println!("Failed to map page: {:?}", e),
    }

    println!("Memory management initialized!");
    println!("MelodyOS is running!");

    loop {}
}

// Function called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Test configuration
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}