use core::fmt::{self, Write};

use crate::serial::SERIAL1;
use crate::vga::VGA_WRITER;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::tty::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use super::tty::TtyWriter;
        // &SERIAL1.write_fmt(args).unwrap();
    unsafe {
    TtyWriter(&VGA_WRITER).write_fmt(args).unwrap();
    TtyWriter(&SERIAL1).write_fmt(args).unwrap();
    }
        // (&VGA_WRITER).write_fmt(args).unwrap();
}
