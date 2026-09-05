#![no_std] // do not link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(asm)]


use core::panic::PanicInfo;


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}



