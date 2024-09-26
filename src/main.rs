#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test::runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod qemu;
mod serial;
mod tty;
mod vga;

#[cfg(test)]
mod test;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
