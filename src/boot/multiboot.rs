// Follows the Multiboot2 Specification
// See https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html#OS-image-format

#[repr(C, align(8))]
pub struct Header<const N: usize> {
    // The field ‘magic’ is the magic number identifying the header, which must be the hexadecimal value 0xE85250D6.
    magic: u32,
    // The field ‘architecture’ specifies the Central Processing Unit Instruction Set Architecture. Since ‘magic’
    // isn’t a palindrome it already specifies the endianness ISAs differing only in endianness recieve the same ID.
    // ‘0’ means 32-bit (protected) mode of i386. ‘4’ means 32-bit MIPS.
    architecture: u32,
    // The field ‘header_length’ specifies the Length of Multiboot2 header in bytes including magic fields.
    header_length: u32,
    // The field ‘checksum’ is a 32-bit unsigned value which, when added to the other magic fields
    // (i.e. ‘magic’, ‘architecture’ and ‘header_length’), must have a 32-bit unsigned sum of zero.
    checksum: u32,
    tags: [u16; N],
}

impl<const N: usize> Header<N> {
    pub const fn new(architecture: u32, tags: [u16; N]) -> Header<N> {
        let magic = 0xE85250D6;
        let header_length = size_of::<Header<N>>() as u32;
        let checksum = magic ^ architecture ^ header_length;

        Header {
            magic,
            architecture,
            header_length,
            checksum,
            tags,
        }
    }
}

#[repr(u16)]
pub enum HeaderTagType {
    End = 0,
    InformationRequest = 1,
    Address = 2,
    EntryAddr = 3,
    ConsoleFlags = 4,
    Framebuffer = 5,
    ModuleAlign = 6,
    EFIBs = 7,
    EntryAddrEFI32 = 8,
    EntryAddrEFI64 = 9,
    Relocatable = 10,
}

#[repr(C, align(8))]
pub struct EndTag {
    r#type: HeaderTagType,
    flags: u16,
    size: u32,
}

impl EndTag {
    pub const fn new() -> Self {
        // Tags are terminated by a tag of type ‘0’ and size ‘8’.
        Self {
            r#type: HeaderTagType::End,
            flags: 0,
            size: 8,
        }
    }
}

#[repr(u32)]
pub enum BootInformationType {
    End = 0,
    BootCommandLine = 1,
    BootLoaderName = 2,
    Modules = 3,
    BasicMemoryInfo = 4,
    BIOSBootDevice = 5,
    MemoryMap = 6,
    VBEInfo = 7,
    FramebufferInfo = 8,
    ElfSymbols = 9,
    APMTable = 10,
    EFI32bitSystemTablePtr = 11,
    EFI64bitSystemTablePtr = 12,
    SMBIOSTables = 13,
    ACPIOldRSDP = 14,
    ACPINewRSDP = 15,
    NetworkingInfo = 16,
    EFIMemoryMap = 17,
    EFIBootServicesNotTerminated = 18,
    EFI32bitImageHandlePtr = 19,
    EFI64bitImageHandlePtr = 20,
    ImageLoadBasePhysAddr = 21,
}

#[repr(C, align(8))]
struct InformationRequestTag<const N: usize> {
    r#type: HeaderTagType,
    flags: u16,
    size: u32,
    mbi_tag_types: [BootInformationType; N],
}

impl<const N: usize> InformationRequestTag<N> {
    pub const fn new(types: [BootInformationType; N]) -> Self {
        Self {
            r#type: HeaderTagType::InformationRequest,
            flags: 0,
            size: size_of::<Self>() as u32,
            mbi_tag_types: types,
        }
    }
}

// This information does not need to be provided if the kernel image is in ELF format, but it must be
// provided if the image is in a.out format or in some other format. When the address tag is present
// it must be used in order to load the image, regardless of whether an ELF header is also present.
// Compliant boot loaders must be able to load images that are either in ELF format or contain the
// address tag embedded in the Multiboot2 header.
#[repr(C, align(8))]
pub struct AddressTag {
    r#type: HeaderTagType,
    flags: u16,
    size: u32,
    // Contains the address corresponding to the beginning of the Multiboot2 header — the physical memory
    // location at which the magic value is supposed to be loaded. This field serves to synchronize
    // the mapping between OS image offsets and physical memory addresses.
    header_addr: u32,
    // Contains the physical address of the beginning of the text segment. The offset in the OS image
    // file at which to start loading is defined by the offset at which the header was found, minus
    // (header_addr - load_addr). load_addr must be less than or equal to header_addr.
    // Special value -1 means that the file must be loaded from its beginning.
    load_addr: u32,
    // Contains the physical address of the end of the data segment. (load_end_addr - load_addr)
    // specifies how much data to load. This implies that the text and data segments must be consecutive
    // in the OS image; this is true for existing a.out executable formats. If this field is zero, the
    // boot loader assumes that the text and data segments occupy the whole OS image file.
    load_end_addr: u32,
    // Contains the physical address of the end of the bss segment. The boot loader initializes this
    // area to zero, and reserves the memory it occupies to avoid placing boot modules and other data
    // relevant to the operating system in that area. If this field is zero, the boot loader assumes
    // that no bss segment is present.
    bss_end_addr: u32,
}
