#![no_main]

use std::fs::File;
use std::io::Write;
use std::os::unix::io::FromRawFd;
use std::path::PathBuf;

#[no_mangle]
pub fn main(_argc: i32, _argv: *const *const u8) {
    let mut stdout = stdout();
    stdout.write(b"home: ");
    stdout.write(str(dirs::home_dir().as_ref()).as_bytes());
    stdout.write(b"\ncache: ");
    stdout.write(str(dirs::cache_dir().as_ref()).as_bytes());
    stdout.write(b"\nconfig: ");
    stdout.write(str(dirs::config_dir().as_ref()).as_bytes());
    stdout.write(b"\ndata: ");
    stdout.write(str(dirs::data_dir().as_ref()).as_bytes());
    stdout.write(b"\ndata_local: ");
    stdout.write(str(dirs::data_local_dir().as_ref()).as_bytes());
    stdout.write(b"\nexecutable: ");
    stdout.write(str(dirs::executable_dir().as_ref()).as_bytes());
    stdout.write(b"\npreferences: ");
    stdout.write(str(dirs::preference_dir().as_ref()).as_bytes());
    stdout.write(b"\nruntime: ");
    stdout.write(str(dirs::runtime_dir().as_ref()).as_bytes());
    stdout.write(b"\nstate: ");
    stdout.write(str(dirs::state_dir().as_ref()).as_bytes());
    stdout.write(b"\naudio: ");
    stdout.write(str(dirs::audio_dir().as_ref()).as_bytes());
    stdout.write(b"\ndesktop: ");
    stdout.write(str(dirs::desktop_dir().as_ref()).as_bytes());
    stdout.write(b"\ndocument: ");
    stdout.write(str(dirs::document_dir().as_ref()).as_bytes());
    stdout.write(b"\ndownload: ");
    stdout.write(str(dirs::download_dir().as_ref()).as_bytes());
    stdout.write(b"\nfont: ");
    stdout.write(str(dirs::font_dir().as_ref()).as_bytes());
    stdout.write(b"\npicture: ");
    stdout.write(str(dirs::picture_dir().as_ref()).as_bytes());
    stdout.write(b"\npublic_dir: ");
    stdout.write(str(dirs::public_dir().as_ref()).as_bytes());
    stdout.write(b"\ntemplate_dir: ");
    stdout.write(str(dirs::template_dir().as_ref()).as_bytes());
    stdout.write(b"\nvideo_dir: ");
    stdout.write(str(dirs::video_dir().as_ref()).as_bytes());
    stdout.write(b"\n");
}

fn str(path: Option<&PathBuf>) -> &str {
    return path.and_then(|p| p.as_os_str().to_str()).unwrap_or("<NONE>");
}

fn stdout() -> File {
    unsafe { File::from_raw_fd(1) }
}
