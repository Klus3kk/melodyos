#![no_std]

// Panic
use core::panic::PanicInfo;

// Function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { // ! - never
    loop{}
}

fn main() {}
