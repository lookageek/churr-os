#![no_std]  // do not link the rust standard library and transitively C runtime from host OS
#![no_main] // disable rust level entry point called by C runtime crt0

use core::panic::PanicInfo;

// this function is called on panic, defining our own because std library of rust provides this
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // do not mangle the name of this function, as it will be used by our linker as entry point
pub extern "C" fn _start() -> ! {
    loop {}
}