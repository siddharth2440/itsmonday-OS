#![no_std]
#![no_main]

use core::panic::PanicInfo;

use rustyos::{exit_qemu, serial_println};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(rustyos::QemuExitCode::Failed);
    loop {}
}



fn test_should_fail(){
    serial_println!("should_panic.....\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(rustyos::QemuExitCode::Success);
    loop {}
}