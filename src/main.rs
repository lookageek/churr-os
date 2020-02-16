#![no_std]  // do not link the rust standard library and transitively C runtime from host OS
#![no_main] // disable rust level entry point called by C runtime crt0

mod vga_buffer;

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
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    loop {}
}