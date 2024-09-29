#![no_main]
#![no_std]

extern crate alloc;

mod location;

use alloc::vec::Vec;
use elf::endian::AnyEndian;
use elf::ElfBytes;
use location::KERNEL_LOCATION;
use uefi::fs::{FileSystem, Path, SEPARATOR};
use uefi::{
    boot::get_image_file_system,
    prelude::*,
    print,
    println,
    CString16
};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let kernel = load_binary(KERNEL_LOCATION);
    parse_elf(&kernel);

    boot::stall(100_000_000);

    Status::SUCCESS
}

fn load_binary(location: &str) -> Vec<u8> {
    let image = boot::image_handle();
    let mut fs = FileSystem::new(get_image_file_system(image).unwrap());

    print!("Looking for kernel binary at '{}'", location);

    let mut location = CString16::try_from(location).unwrap();
    let backslash = cstr16!("/").as_slice()[0];
    location.replace_char(backslash, SEPARATOR);

    let path = Path::new(&location);

    if let Ok(exists) = fs.try_exists(path) {
        if exists {
            println!("\rFound the kernel binary at '{}'!          ", location);
        } else {
            panic!("\rKernel binary not found at '{}'!            ", location);
        }
    }

    print!("Reading kernel binary...");
    let kernel = fs.read(path).unwrap();
    println!("\rRead kernel binary of {} bytes from '{}'!", kernel.len(), location);
    kernel
}

fn parse_elf(bytes: &[u8]) {
    let elf = match ElfBytes::<AnyEndian>::minimal_parse(bytes) {
        Ok(elf) => elf,
        Err(err) => panic!("Failed to parse kernel image. Are you sure it is a valid ELF file? Error: {err}")
    };
}
