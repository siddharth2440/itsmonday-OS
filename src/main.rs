#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod vga_buffer;
// pub mod serial;

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
    // println!("Hello It'sMoNdAy. How's your day going??");

    #[cfg(test)]
    test_main();

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

    // exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    print!("running test");
    assert_eq!(1,1);
    println!("test passed:  [ok]")
}

#[test_case]
fn trivial_assertion1() {
    print!("running test");
    assert_eq!(1,1);
    println!("test passed:  [ok]")
}

#[test_case]
fn trivial_assertion2() {
    print!("running test");
    assert_eq!(1,1);
    println!("test passed:  [ok]")
}


// QemuExit Code 
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum QemuExitCode {
//     Success = 0x10,
//     Failed = 0x11
// }

// pub fn exit_qemu( exit_code: QemuExitCode ) {
//     use x86_64::instructions::port::Port;
//     unsafe {
//         let mut port = Port::new(0xf4);
//         port.write(exit_code as u32);
//     }
// }