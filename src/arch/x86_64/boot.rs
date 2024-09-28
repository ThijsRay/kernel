use crate::boot::multiboot::Header;

#[link_section = ".text.multiboot"]
#[no_mangle]
static MULTIBOOT_HEADER: Header<0> = Header::new(0, []);
