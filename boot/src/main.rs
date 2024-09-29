#![no_main]
#![no_std]

use uefi::{boot::SearchType, prelude::*, println};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let handles = boot::locate_handle_buffer(SearchType::AllHandles).expect("Failed to locate all handles");
    for handle in handles.into_iter() {
        println!("handle @ {:?}", handle.as_ptr());
    }

    Status::SUCCESS
}
