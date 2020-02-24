#![no_std]
#![no_main]

// all integration tests should themselves defined these things as they are all executables themselves
// - custom test framework initialization
// - main function
// - panic handler
// - test runner

#![feature(custom_test_frameworks)]
#![test_runner(churr_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use churr_os::{println, serial_print, serial_println};

// don't mangle the name of this function as linker will look for this name
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

// we do not need #[cfg(test)] attribute for this panic handler, as integration tests are only
// compiled in test mode of compiler run
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    churr_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    serial_print!("test_println... ");
    println!("test_println output");
    serial_println!("[ok]");
}