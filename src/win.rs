extern crate windows_sys as windows;

use self::windows::Win32::Foundation::HANDLE;
use self::windows::Win32::Storage::FileSystem;
use self::windows::Win32::System::Console;

use std::ffi::c_void;
use std::io::{Error, Result};
use std::ptr::null_mut;

pub(crate) fn stdout() -> HANDLE {
     unsafe { Console::GetStdHandle(Console::STD_OUTPUT_HANDLE) }
}

pub(crate) fn write(handle: &HANDLE, buf: &[u8]) -> Result<usize> {
    if buf.len() == 0 {
        Ok(0)
    } else if is_console(handle) {
        write_to_console(handle, buf)
    } else {
        write_to_file(handle, buf)
    }
}

fn is_console(handle: &HANDLE) -> bool {
    let mut mode = 0;
    unsafe { Console::GetConsoleMode(*handle, &mut mode) != 0 }
}

fn write_to_console(handle: &HANDLE, buf: &[u8]) -> Result<usize> {
    let mut bytes_written = 0;
    unsafe {
        if Console::WriteConsoleA(
            *handle,
            buf.as_ptr() as *const c_void,
            buf.len() as u32,
            &mut bytes_written,
            null_mut(),
        ) == 0 {
            Err(Error::last_os_error())
        } else {
            assert_eq!(bytes_written, buf.len() as u32);
            Ok(bytes_written as usize)
        }
    }
}

fn write_to_file(handle: &HANDLE, buf: &[u8]) -> Result<usize> {
    let buf = match String::from_utf8(buf.to_vec()) {
        Ok(string) => string,
        Err(e) => return Err(Error::new(std::io::ErrorKind::InvalidInput, e)),
    };

    let mut bytes_written = 0;

    unsafe {
        if FileSystem::WriteFile(
            *handle,
            buf.as_ptr(),
            buf.len() as u32,
            &mut bytes_written,
            null_mut(),
        ) == 0 {
            return Err(Error::last_os_error());
        }
    }
    return Ok(buf.len());
}
