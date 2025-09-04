#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

pub mod serial;
pub mod vga_buffer;
pub mod idt;
pub mod interrupts;
pub mod gdt;


// ---------------------------------- Qemu ---------------------------------- 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11
}

pub fn exit_qemu( exit_code: QemuExitCode ) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
// ----------------------------------- Qemu --------------------------------------


pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn()
{
    fn run(&self) {
        serial_println!("{}...\t", core::any::type_name::<T>());
        self();  // i didn't get this line
        serial_println!("[ok]");
    }
}

// #[cfg(test)]
pub fn test_runner( tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());

    for _test in tests {
        _test.run(); 
    }

    // Qemu Exit
    exit_qemu(QemuExitCode::Success);
}


pub fn  test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// #[test_case]
// fn trivial_assertion() {
//     assert_eq!(1,1);
// }

// #[test_case]
// fn trivial_assertion1() {
//     println!("test_println_simple_output");
// }


// #[test_case]
// fn print_many_line(){
//     for num in 1..=200 {
//         println!("printing {:?}", num);
//     }
// }

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

// IDT 
pub fn init() {
    interrupts::init_idt();
}



// TSS