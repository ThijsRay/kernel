use super::color::ColorCode;
use super::{BUFFER_HEIGHT, BUFFER_WIDTH};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub(super) struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}

#[repr(transparent)]
pub(super) struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Buffer {
    pub fn write(&mut self, row: usize, column: usize, char: ScreenChar) {
        let cell = &mut self.chars[row][column];
        let pointer = unsafe { volatile::VolatilePtr::new(cell.into()) };

        pointer.write(char);
    }

    pub fn read(&self, row: usize, column: usize) -> ScreenChar {
        let cell = &self.chars[row][column];
        let pointer = unsafe { volatile::VolatilePtr::new_read_only(cell.into()) };
        pointer.read()
    }
}
