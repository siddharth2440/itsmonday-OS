// #![test(crate::test_runner)]
#![no_std]
#![no_main]
// #![feature(custom_test_frameworks)]

use core::fmt::Write;

pub mod vga_buffer;

static HELLO: &[u8] = b"                                  It'sMoNdAy OS                                                                                                                  ";
#[unsafe(no_mangle)]
pub extern "C" fn _start() {

    let vga_buffer = 0xb8000 as *mut u8;
    for ( i, &byte ) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    println!("Hello It'sMoNdAy. How's your day going??");

    loop {}
}

#[panic_handler]
fn panic( _info: &core::panic::PanicInfo ) -> ! {
    println!("{}", _info);
    loop {}
}


#[cfg(test)]
pub fn test_runner( tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());

    for _test in tests {
        _test();
    }
}