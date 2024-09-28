#![no_std] // Don't link the Rust standard library
#![no_main] // Because we don't want to use main

// Panic
use core::panic::PanicInfo;
mod vga_buffer;



// Function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { // ! - never
    loop{}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // Disabling name mangling
pub extern "C" fn _start() -> ! { // Entry point
    let vga_buffer = 0xb8000 as *mut u8; // Raw pointer

    for (i, &byte) in HELLO.iter().enumerate() { // Iterate over the bytes of the static HELLO byte string
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // Using offset to write the string byte and the corresponding color byte (0xb is light cyan)
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop{}
}