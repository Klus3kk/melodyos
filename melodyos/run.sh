#!/bin/bash
cargo clean
cargo +nightly build
cargo +nightly bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-melody_os/debug/bootimage-melodyos.bin