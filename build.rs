use std::str::FromStr;

enum Arch {
    X86_64
}

#[derive(Debug)]
struct UnknownArchError;

impl FromStr for Arch {
    type Err = UnknownArchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arch = match s {
            "x86_64" => Arch::X86_64,
            _ => Err(UnknownArchError)?
        };
        Ok(arch)
    }
}

impl ToString for Arch {
    fn to_string(&self) -> String {
        match self {
            Arch::X86_64 => "x86_64",
        }.into()
    }
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    let arch = Arch::from_str(&std::env::var("CARGO_CFG_TARGET_ARCH").unwrap()).unwrap();
    generate_linker_args(arch)
}

fn generate_linker_args(arch: Arch) {
    let args = match arch {
        Arch::X86_64 => generate_x86_64_linker_args(),
    };

    for arg in args {
        println!("cargo::rustc-link-arg={arg}")
    }
}

fn generate_x86_64_linker_args() -> Vec<String> {
    let args = Vec::new();
    args
}
