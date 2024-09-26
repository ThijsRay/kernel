use core::fmt;
use core::ops::Deref;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    {
        let writer_lock = super::writer::WRITER.lock();
        let cell = writer_lock.deref().deref();
        let writer = unsafe { cell.get().as_mut().unwrap() };

        writer.write_fmt(args).unwrap();
    }
}
