#![no_std]
#![no_main]

// This integration test has its test harness set to false in Cargo.toml, so we don't have to initialise,
// custom test framework and a test runner, but we call the test directly from main, useful for
// scenarios having specific requirements of test ordering or just simply there is just one test to run

use core::panic::PanicInfo;
use churr_os::{QemuExitCode, exit_qemu, serial_println, serial_print};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    // if the test did not panic and force main function to exit,
    // fail the test, exit with Failed code
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// panic handler exits with Success exit code out of QEMU emulator
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

// A panicky test, assert_eq will panic
fn should_fail() {
    serial_print!("should_fail... ");
    assert_eq!(0, 1);
}