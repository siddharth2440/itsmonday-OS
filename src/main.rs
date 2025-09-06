#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustyos::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

#[cfg(test)]
use core::panic::PanicInfo;

pub mod vga_buffer;
pub mod serial;
pub mod idt;
pub mod interrupts;
pub mod ist;
pub mod segmentation;

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

    rustyos::init();

    println!("Hello It'sMoNdAy. How's your day going??");

    // triget page fault 
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // }

    // invoking breakpoint exception
    // x86_64::instructions::interrupts::int3();


    // for stack overflow
    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // stack_overflow();

    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    
    #[cfg(test)]
    test_main();
    
    println!("Hello It'sMoNdAy. How's your day going??");
    loop {}
}

// panic handler 
#[cfg(not(test))]
#[panic_handler]
fn panic( _info: &core::panic::PanicInfo ) -> ! {
    println!("{}", _info);
    loop {}
}

// test (panic handler)
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustyos::test_panic_handler(info)
}


#[test_case]
fn triavial_assert() {
    assert_ne!(1,0);
}