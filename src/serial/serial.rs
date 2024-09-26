use spin::{Lazy, Mutex};
use uart_16550::SerialPort;

use crate::tty::Tty;

pub static mut SERIAL1: Mutex<Lazy<SerialPort>> = Mutex::new(Lazy::new(|| {
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.init();
    serial_port
}));

impl Tty for SerialPort {
    fn read<A: AsMut<[u8]>>(&self, buf: A) -> usize {
        todo!()
    }

    fn write<A: AsRef<[u8]>>(&mut self, bytes: A) -> usize {
        for byte in bytes.as_ref().into_iter() {
            self.send_raw(*byte)
        }
        bytes.as_ref().len()
    }
}
