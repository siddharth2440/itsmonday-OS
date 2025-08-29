#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use rustyos::{exit_qemu, serial_println};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(rustyos::QemuExitCode::Failed);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(rustyos::QemuExitCode::Success);
    loop {}
}


fn test_should_fail(){
    serial_println!("should_panic.....\t");
    assert_eq!(0, 1);
}