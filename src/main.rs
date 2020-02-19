#![no_std]  // do not link the rust standard library and transitively C runtime from host OS
#![no_main] // disable rust level entry point called by C runtime crt0

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;

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
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// do not mangle the name of this function,
// as it will be used by our linker as entry point
// this function is the entry_point,
// linker looks specifically for a function named `_start`
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    // call the test suite entry point, redefined the name to test_main in attr macro
    #[cfg(test)]
    test_main();

    loop {}
}

// exit code struct for qemu VM which will be written to isa-debug-exit device
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

// write the exit code to port of isa-debug-exit device, thus making qemu exit
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// test suite runner, iterates over all functions marked #[test_case]
// and runs them, exits qemu VM after that
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}
