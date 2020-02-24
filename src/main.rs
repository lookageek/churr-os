#![no_std]  // do not link the rust standard library and transitively C runtime from host OS
#![no_main] // disable rust level entry point called by C runtime crt0

// importing and using many functions in lib crate

#![feature(custom_test_frameworks)]
#![test_runner(churr_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use churr_os::println;

// this function is called on panic,
// defining our own because std library of rust provides this
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    churr_os::test_panic_handler(info)
}

// do not mangle the name of this function,
// as it will be used by our linker as entry point
// this function is the entry_point,
// linker looks specifically for a function named `_start`
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    // call the test suite entry point, redefined the name to test_main in attr macro,
    // compile and execute this method only in test compilation mode
    #[cfg(test)]
    test_main();

    loop {}
}