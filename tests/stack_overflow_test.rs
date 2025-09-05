#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use lazy_static::lazy_static;
use rustyos::{exit_qemu, serial_print, serial_println};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};


pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow_test\t");

    rustyos::gdt::init();
    init_test_idt();

    stack_overflow();
    panic!("Execution continues after stack overflow");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustyos::test_panic_handler(info)
}


#[allow(unconditional_recursion)]
fn stack_overflow(){
    stack_overflow();
    volatile::Volatile::new(0).read();
}


lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(rustyos::gdt::DOUBLE_FAULT_1ST_INDEX);
        }

        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}


extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame, _error_code: u64
) -> ! {
    serial_println!("[ok]");
    exit_qemu(rustyos::QemuExitCode::Success);
    loop {}
}