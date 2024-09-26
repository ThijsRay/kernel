use core::error::Error;
use core::fmt::Display;

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

#[derive(Debug)]
pub(super) enum VGABufferWriteError {
    RowOutOfBounds(usize),
    ColumnOfOfBounds(usize),
}
impl Error for VGABufferWriteError {}
impl Display for VGABufferWriteError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use VGABufferWriteError::*;
        match self {
            RowOutOfBounds(x) => write!(
                f,
                "Row is out of bounds, must be smaller than {BUFFER_WIDTH} but is {x}."
            ),
            ColumnOfOfBounds(x) => write!(
                f,
                "Column is out of bounds, must be smaller than {BUFFER_HEIGHT} but is {x}."
            ),
        }
    }
}

impl Buffer {
    pub fn write(
        &mut self,
        row: usize,
        column: usize,
        char: ScreenChar,
    ) -> Result<(), VGABufferWriteError> {
        if row >= BUFFER_WIDTH {
            return Err(VGABufferWriteError::RowOutOfBounds(row));
        }

        if column >= BUFFER_HEIGHT {
            return Err(VGABufferWriteError::ColumnOfOfBounds(column));
        }

        let cell = &mut self.chars[row][column];
        let pointer = unsafe { volatile::VolatilePtr::new(cell.into()) };

        pointer.write(char);
        Ok(())
    }
}
