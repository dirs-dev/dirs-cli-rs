use std::fs::File;
use std::io::Write;

use std::os::unix::io::FromRawFd;

pub(crate) fn stdout() -> File {
    unsafe { File::from_raw_fd(1) }
}

pub(crate) fn write(console: &mut File, buf: &[u8]) {
    console.write(buf);
}
