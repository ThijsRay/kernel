use core::{
    cell::{LazyCell, UnsafeCell},
    fmt,
};

use spin::Mutex;

use super::{
    buffer::{Buffer, ScreenChar},
    color::{Color, ColorCode},
    BUFFER_HEIGHT, BUFFER_WIDTH,
};

pub static WRITER: Mutex<LazyCell<UnsafeCell<Writer>>> = Mutex::new(LazyCell::new(|| {
    UnsafeCell::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    })
}));

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
                );

                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for column in 0..BUFFER_WIDTH {
                let character = self.buffer.read(row, column);
                self.buffer.write(row - 1, column, character);
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let space = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        for column in 0..BUFFER_WIDTH {
            self.buffer.write(row, column, space);
        }
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

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
