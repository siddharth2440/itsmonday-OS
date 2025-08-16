#![no_std]
#![no_main]

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
    vga_buffer::testing_writer();

    loop {}
}

#[panic_handler]
fn panic( _info: &core::panic::PanicInfo ) -> ! {
    loop {}
}