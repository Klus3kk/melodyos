#![no_std] // Don't link the Rust standard library
#![no_main] // Because we don't want to use main

// Panic
use core::panic::PanicInfo;
mod vga_buffer;

#[no_mangle] // Disabling name mangling
pub extern "C" fn _start() -> ! { // Entry point
    println!("Hello World{}", "!");
    panic!("Some panic message");
    loop {}
}

// Function called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! { // ! - never
    println!("{}", info);
    loop {}
}