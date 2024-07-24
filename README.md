# MelodyOS
This project aims to develop a simple operating system written entirely in Rust. It serves as an educational tool for understanding low-level programming concepts, operating system internals, and the Rust programming language's capabilities in system programming. The project covers basic OS functionalities including bootstrapping, memory management, task scheduling, and simple file system implementation.
## Features
* Bootloader: Custom bootloader to initialize the hardware and load the kernel.
* Kernel: Basic kernel written in Rust, handling essential OS tasks.
* Memory Management: Simple memory allocation and deallocation mechanisms.
* Task Scheduling: Basic round-robin task scheduler to manage multiple processes.
* File System: Minimalistic file system for storing and retrieving data.
* Interrupt Handling: Mechanisms for handling hardware and software interrupts.
* Rust’s Safety: Leveraging Rust's safety features to prevent common bugs such as buffer overflows and null pointer dereferences.
## Technologies 
* Programming Language:
- Rust: Core language for OS development, chosen for its safety and performance.

**Bootloader:**
- x86 Assembly: Used for writing the initial bootloader code.
- GRUB: Common bootloader to load the Rust kernel.
  
**Development Tools:**
- QEMU: Emulator for testing the OS without needing physical hardware.
- Cargo: Rust's package manager and build system.
- LLVM: Compiler infrastructure used by Rust.

**Libraries:**
- x86_64: Rust crate for x86_64 architecture support.
- Bootimage: Tool to create bootable disk images for Rust OS kernels.
- Spin: Provides spinlock-based synchronization primitives.
