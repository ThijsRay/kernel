#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

pub extern "C" fn _start() -> ! {
    unsafe { asm!("int3") }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
