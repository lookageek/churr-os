#![no_std]  // do not link the rust standard library and transitively C runtime from host OS
#![no_main] // disable rust level entry point called by C runtime crt0

use core::panic::PanicInfo;

// this function is called on panic, defining our own because std library of rust provides this
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"HELLO WORLD!";

#[no_mangle] // do not mangle the name of this function, as it will be used by our linker as entry point
// this function is the entry_point, linker looks specifically for a function named `_start`
pub extern "C" fn _start() -> ! {
    let vga_buffer_addr = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer_addr.offset(i as isize * 2) = byte;
            *vga_buffer_addr.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}