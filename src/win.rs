extern crate win32console;

use win::win32console::console::WinConsole;

pub(crate) fn stdout() -> WinConsole {
    WinConsole::output()
}

pub(crate) fn write(console: &WinConsole, buf: &[u8]) {
    console.write_utf8(buf);
}
