pub mod popol;

use std::os::unix::io::{AsRawFd, RawFd};
use std::time::Duration;

/// Information about I/O events which has happened for an actor
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct IoEv {
    /// Specifies whether I/O source has data to read.
    pub is_readable: bool,
    /// Specifies whether I/O source is ready for write operations.
    pub is_writable: bool,
}

pub trait Poll
where
    Self: Send + Iterator<Item = (RawFd, IoEv)>,
    for<'a> &'a mut Self: Iterator<Item = (RawFd, IoEv)>,
{
    fn register(&mut self, fd: impl AsRawFd);
    fn unregister(&mut self, fd: impl AsRawFd);

    fn poll(&mut self) -> (Duration, usize);
}
