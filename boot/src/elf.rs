use core::fmt::Display;

use elf::{endian::AnyEndian, file::{Class, FileHeader}, ElfBytes};

pub(crate) fn read(bytes: &[u8]) {
    let elf = match ElfBytes::<AnyEndian>::minimal_parse(bytes) {
        Ok(elf) => elf,
        Err(err) => panic!(
            "Failed to parse kernel image. Are you sure it is a valid ELF file? Error: {err}"
        ),
    };

    validate_headers(elf.ehdr).unwrap();
    ()
}

#[derive(Debug, PartialEq, Eq)]
enum ElfFileType {
    None,
    Relocatable,
    Executable,
    SharedObject,
    Core
}
impl From<u16> for ElfFileType {
    fn from(value: u16) -> Self {
        use ElfFileType::*;
        match value {
            0 => None,
            1 => Relocatable,
            2 => Executable,
            3 => SharedObject,
            4 => Core,
            e => panic!("Unknown ELF file type: {e}")
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ElfArchitecture {
    None,
    X86_64
}
impl From<u16> for ElfArchitecture {
    fn from(value: u16) -> Self {
        use ElfArchitecture::*;
        match value {
            0 => None,
            62 => X86_64,
            e => panic!("Unknown ELF architecture: {e}")
        }
    }
}

#[derive(Debug)]
enum ElfHeaderValidationError {
    WordSize(Class),
    Endianness(AnyEndian),
    Architecture(ElfArchitecture),
    FileType(ElfFileType)
}
impl core::error::Error for ElfHeaderValidationError {}
impl Display for ElfHeaderValidationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ElfHeaderValidationError::WordSize(x) => write!(f, "Word size should not be {x:?} for this architecture"),
            ElfHeaderValidationError::Endianness(x) => write!(f, "Endianness should not be {x:?} for this architecture"),
            ElfHeaderValidationError::Architecture(x) => write!(f, "Incorrect architecture found: {x:?}"),
            ElfHeaderValidationError::FileType(x) => write!(f, "File must be of executable type, but is {x:?}"),
        }
    }
}

fn validate_headers(
    header: FileHeader<AnyEndian>,
) -> Result<FileHeader<AnyEndian>, ElfHeaderValidationError> {
    let filetype = ElfFileType::from(header.e_type);
    if filetype != ElfFileType::Executable {
        return Err(ElfHeaderValidationError::FileType(filetype))
    }

    #[cfg(target_arch = "x86_64")]
    validate_headers_x86_64(header)
}

#[cfg(target_arch = "x86_64")]
fn validate_headers_x86_64(
    header: FileHeader<AnyEndian>,
) -> Result<FileHeader<AnyEndian>, ElfHeaderValidationError> {
    use ElfHeaderValidationError::*;

    if header.class != Class::ELF64 {
        return Err(WordSize(header.class));
    }

    if header.endianness != AnyEndian::Little {
        return Err(Endianness(header.endianness))
    }

    let architecture = ElfArchitecture::from(header.e_machine);
    if architecture != ElfArchitecture::X86_64 {
        return Err(Architecture(architecture))
    }

    Ok(header)
}
