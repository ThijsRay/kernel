use core::fmt;

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
    use core::fmt::Write;
    unsafe {
        SERIAL1.write_fmt(args);
        VGA_WRITER.write_fmt(args);
    }
}
