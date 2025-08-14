#[repr(u8)]
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


#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct ColorCode(u8);

// 1 byte ColorCode (0-7)
#[allow(dead_code)]
impl ColorCode {
    fn new( f_ground: Color, b_ground: Color ) -> Self {
        return Self( ((b_ground as u8) << 4) | ( f_ground as u8 ) );
    }
}

// 16 byte ----  0-7(character) 7-15(colors ie., foreground, background, blink )
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct ScreenCharacter{
    ascii_ch: u8,
    color_code: ColorCode
}

#[allow(dead_code)]
const BUFFER_HEIGHT:usize = 25;

#[allow(dead_code)]
const BUFFER_WIDTH:usize = 80;

#[allow(dead_code)]
struct BUFFER {
    chars: [ [ ScreenCharacter; BUFFER_WIDTH ]; BUFFER_HEIGHT ]
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
            b'\n' => unimplemented!(),
            byte => {
                if self.col_pos >= BUFFER_WIDTH {
                    unimplemented!();
                }

                let row: usize = BUFFER_HEIGHT - 1;
                let col: usize = self.col_pos;

                let _color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenCharacter { ascii_ch: byte, color_code: _color_code };
                self.col_pos += 1;
            }
        }
    }
}