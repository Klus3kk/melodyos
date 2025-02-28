fn main() {
    // Tell cargo to rerun this build script if build.rs changes
    println!("cargo:rerun-if-changed=build.rs");
    
    // Tell cargo to pass specific flags to the bootloader crate
    println!("cargo:rustc-env=RUSTFLAGS=--cfg bootloader_no_soft_float");
}