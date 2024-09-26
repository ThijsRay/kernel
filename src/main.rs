#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::print_something();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
