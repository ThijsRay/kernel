#![no_std]
#![no_main]

mod vga;

use core::{ops::Deref, panic::PanicInfo};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    {
        let writer_lock = vga::WRITER.lock();
        let cell = writer_lock.deref().deref();
        let writer = unsafe { cell.get().as_mut().unwrap() };

        write!(writer, "Hello again!").unwrap();
        write!(writer, ", some numbers: {} {}", 42, 1.337).unwrap();
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
