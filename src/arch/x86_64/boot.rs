use core::arch::asm;

use crate::main;

#[no_mangle]
pub extern "C" fn efi_main(_h: *mut ::core::ffi::c_void, _st: *mut core::ffi::c_void) -> usize {
    unsafe { asm!("int3"); }
    main();
    0
}
