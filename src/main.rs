#![no_std]
#![no_main]

mod arch;

use core::{arch::asm, panic::PanicInfo};

pub fn main() {
    unsafe {
        asm!("int3");
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
