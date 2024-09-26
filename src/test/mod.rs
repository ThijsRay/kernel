use crate::qemu;

pub fn runner(tests: &[&dyn Fn()]) {
    crate::println!("Runing {} tests", tests.len());
    for test in tests {
        test();
    }

    qemu::exit(qemu::QemuExitCode::Success);
}
