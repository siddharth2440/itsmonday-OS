#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustyos::test_runner)]
#![reexport_test_harness_main="test_main"]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

fn test_runner( _tests: &[&dyn Fn()]) {
    unimplemented!();
}

fn panic( _info: &PanicInfo) -> ! {
    loop{}
}