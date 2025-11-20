#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustyos::test_runner)]
#![reexport_test_harness_main="test_main"]

use core::panic::PanicInfo;

use rustyos::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    
    loop {}
}

// fn test_runner( _tests: &[&dyn Fn()]) {
//     unimplemented!();
// }

#[panic_handler]
fn panic( _info: &PanicInfo) -> ! {
    rustyos::test_panic_handler(_info);
}

#[test_case]
fn test_println() {
    println!("test pass: [ok]");
    assert_eq!(1, 0);
}