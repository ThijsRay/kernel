use super::{
    buffer::{Buffer, ScreenChar},
    color::{Color, ColorCode},
    BUFFER_HEIGHT, BUFFER_WIDTH,
};

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.write(
                    row,
                    col,
                    ScreenChar {
                        ascii_character: byte,
                        color_code,
                    },
                ).unwrap();

                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        unimplemented!();
    }

    pub fn write_string(&mut self, s: impl AsRef<str>) {
        for byte in s.as_ref().bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
}

pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld");
}
