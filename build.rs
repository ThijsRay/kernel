use std::str::FromStr;

enum Arch {
    X86_64,
}

#[derive(Debug)]
struct UnknownArchError;

impl FromStr for Arch {
    type Err = UnknownArchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arch = match s {
            "x86_64" => Arch::X86_64,
            _ => Err(UnknownArchError)?,
        };
        Ok(arch)
    }
}

impl ToString for Arch {
    fn to_string(&self) -> String {
        match self {
            Arch::X86_64 => "x86_64",
        }
        .into()
    }
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-link-arg=--subsystem,10");

    // let arch = Arch::from_str(&std::env::var("CARGO_CFG_TARGET_ARCH").unwrap()).unwrap();
    // generate_linker_args(arch)
}

fn generate_linker_args(arch: Arch) {
    let path = format!("src/arch/{}/linker.ld", arch.to_string());
    println!("cargo::rerun-if-changed={path}");
    println!("cargo::rustc-link-arg=-T{path}")
}
