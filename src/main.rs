#![no_main]

#[cfg(not(target_os = "windows"))]
mod unix;
#[cfg(target_os = "windows")]
mod win;

#[cfg(not(target_os = "windows"))]
use unix::{stdout, write};

#[cfg(target_os = "windows")]
use win::{ stdout, write };

use std::path::PathBuf;
use std::process::exit;

#[allow(unused)]
#[no_mangle]
pub fn main(_argc: i32, _argv: *const *const u8) {
    let mut stdout = stdout();
    write(&mut stdout, b"home:         ");
    write(&mut stdout, str(dirs::home_dir().as_ref()).as_bytes());
    write(&mut stdout, b"\ncache:        ");
    write(&mut stdout, str(dirs::cache_dir().as_ref()).as_bytes());
    write(&mut stdout, b"\nconfig:       ");
    write(&mut stdout, str(dirs::config_dir().as_ref()).as_bytes());
    write(&mut stdout, b"\nconfig_local: ");
    write(&mut stdout, str(dirs::config_local_dir().as_ref()).as_bytes());
    write(&mut stdout, b"\ndata:         ");
    write(&mut stdout, str(dirs::data_dir().as_ref()).as_bytes());
    write(&mut stdout, b"\ndata_local:   ");
    write(&mut stdout, str(dirs::data_local_dir().as_ref()).as_bytes());
    write(&mut stdout, b"\nexecutable:   ");
    write(&mut stdout, str(dirs::executable_dir().as_ref()).as_bytes());
    write(&mut stdout, b"\npreferences:  ");
    write(&mut stdout, str(dirs::preference_dir().as_ref()).as_bytes());
    write(&mut stdout, b"\nruntime:      ");
    write(&mut stdout, str(dirs::runtime_dir().as_ref()).as_bytes());
    write(&mut stdout, b"\nstate:        ");
    write(&mut stdout, str(dirs::state_dir().as_ref()).as_bytes());
    write(&mut stdout, b"\naudio:        ");
    write(&mut stdout, str(dirs::audio_dir().as_ref()).as_bytes());
    write(&mut stdout, b"\ndesktop:      ");
    write(&mut stdout, str(dirs::desktop_dir().as_ref()).as_bytes());
    write(&mut stdout, b"\ndocument:     ");
    write(&mut stdout, str(dirs::document_dir().as_ref()).as_bytes());
    write(&mut stdout, b"\ndownload:     ");
    write(&mut stdout, str(dirs::download_dir().as_ref()).as_bytes());
    write(&mut stdout, b"\nfont:         ");
    write(&mut stdout, str(dirs::font_dir().as_ref()).as_bytes());
    write(&mut stdout, b"\npicture:      ");
    write(&mut stdout, str(dirs::picture_dir().as_ref()).as_bytes());
    write(&mut stdout, b"\npublic:       ");
    write(&mut stdout, str(dirs::public_dir().as_ref()).as_bytes());
    write(&mut stdout, b"\ntemplate      ");
    write(&mut stdout, str(dirs::template_dir().as_ref()).as_bytes());
    write(&mut stdout, b"\nvideo         ");
    write(&mut stdout, str(dirs::video_dir().as_ref()).as_bytes());
    write(&mut stdout, b"\n");
    exit(0);
}

fn str(path: Option<&PathBuf>) -> &str {
    return path.and_then(|p| p.as_os_str().to_str()).unwrap_or("<NONE>");
}
