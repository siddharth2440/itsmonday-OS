#[repr(u8)]
#[derive(Debug,Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Read = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DaryGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15
}

// #[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

// 1 byte ColorCode (0-7)
#[allow(dead_code)]
impl ColorCode {
    fn new( f_ground: Color, b_ground: Color ) -> Self {
        return Self( ((b_ground as u8) << 4) | ( f_ground as u8 ) );
    }
}

// 16 byte ----  0-7(character) 7-15(colors ie., foreground, background, blink )
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
// #[allow(dead_code)]
struct ScreenCharacter{
    ascii_ch: u8,
    color_code: ColorCode
}

#[allow(dead_code)]
const BUFFER_HEIGHT:usize = 25;

#[allow(dead_code)]
const BUFFER_WIDTH:usize = 80;


// #[allow(dead_code)]
#[repr(transparent)]
struct BUFFER {
    chars: [[ volatile::Volatile<ScreenCharacter>; BUFFER_WIDTH ]; BUFFER_HEIGHT ]
}


// Writer
pub struct Writer {
    col_pos: usize,
    color_code: ColorCode,
    buffer: &'static mut BUFFER
}


impl Writer {
    pub fn write_byte( &mut self, byte: u8 ) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col_pos >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row: usize = BUFFER_HEIGHT - 1;
                let col: usize = self.col_pos;

                let _color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenCharacter { ascii_ch: byte, color_code: _color_code });
                self.col_pos += 1;
            }
        }
    }

    pub fn new_line(&mut self){
        // TODO

        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let ch = self.buffer.chars[row][col].read();
                self.buffer.chars[row-1][col].write(ch);
            }
        }
        self.clear_row(BUFFER_HEIGHT -1);
        self.col_pos = 0;
    }

    pub fn clear_row( &mut self, row: usize ) {
        let blank: ScreenCharacter = ScreenCharacter { ascii_ch: b' ', color_code: self.color_code };
        for col  in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // ascii byte or newline
                0x20..=0x7e | b'\n' => {
                    self.write_byte(byte)
                },
                _ => self.write_byte(0xfe),
            }
        }
    }

}

use core::fmt;
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// pub fn testing_writer() {
//     let mut writer: Writer = Writer { 
//         col_pos: 0, 
//         color_code: ColorCode::new(Color::Green, Color::Black), 
//         buffer: unsafe { &mut *( 0xb8000 as *mut BUFFER ) }
//     };
//     writer.write_byte(b'W');
//     writer.write_string("elcome It'sMoNdAy!!");
//     // write!( writer, "write line implementaion ok!").unwrap();
// }

lazy_static::lazy_static!{
    pub static ref WRITER: spin::Mutex<Writer> = spin::Mutex::new(Writer {
                                        col_pos: 0,
                                        color_code: ColorCode::new(Color::Green, Color::Black),
                                        buffer: unsafe { &mut *(0xb8000 as *mut BUFFER) },
                                    });
}

#[macro_export]
macro_rules! print  {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}


#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}


#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}


// Tests 
#[test_case]
fn test_println_output() {
    let s = "this is how legends are made";
    println!("{}", s);
    for ( i, c ) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT-2][i].read();
        assert_eq!(char::from(screen_char.ascii_ch), c); 
    }
}