use core::ops::Deref;

use spin::{Lazy, Mutex};

pub trait Tty {
    fn read<A: AsMut<[u8]>>(&self, buf: A) -> usize;
    fn write<A: AsRef<[u8]>>(&mut self, bytes: A) -> usize;
}

pub(crate) struct TtyWriter<T: Tty>(pub T);
impl<T> core::fmt::Write for TtyWriter<T>
where
    T: Tty,
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0.write(s.as_bytes());
        Ok(())
    }
}

impl<T> From<T> for TtyWriter<T>
    where T: Tty {
        fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Tty for &mut T
where
    T: Tty,
{
    fn read<A: AsMut<[u8]>>(&self, buf: A) -> usize {
        (**self).read(buf)
    }

    fn write<A: AsRef<[u8]>>(&mut self, bytes: A) -> usize {
        (**self).write(bytes)
    }
}

impl<T> Tty for &Mutex<Lazy<T>>
where
    T: Tty,
{
    fn read<A: AsMut<[u8]>>(&self, buf: A) -> usize {
        let lock = self.lock();
        lock.read(buf)
    }

    fn write<A: AsRef<[u8]>>(&mut self, bytes: A) -> usize {
        let lock = self.lock();
        {
            (*lock).deref();
        }
        let t_ptr = (*lock).as_mut_ptr();
        let t = unsafe { t_ptr.as_mut() }.unwrap();
        t.write(bytes)
    }
}

impl<T> Tty for Mutex<Lazy<T>>
where
    T: Tty,
{
    fn read<A: AsMut<[u8]>>(&self, buf: A) -> usize {
        let lock = self.lock();
        lock.read(buf)
    }

    fn write<A: AsRef<[u8]>>(&mut self, bytes: A) -> usize {
        let lock = self.lock();
        {
            (*lock).deref();
        }
        let t_ptr = (*lock).as_mut_ptr();
        let t = unsafe { t_ptr.as_mut() }.unwrap();
        t.write(bytes)
    }
}
