use std::path::Path;

fn main() {
    #[cfg(windows)]
    build_windows();
    #[cfg(target_os = "macos")]
    build_macos();
    #[cfg(target_os = "linux")]
    build_linux();
}

#[cfg(windows)]
fn build_windows() {
    cc::Build::new()
        .includes(Path::new("win"))
        .includes(Path::new("common"))
        .flag("-DKEYBOARD_ONLY")
        .files(&[
            "win/rfb_win32/SInput.cxx",
            "vncviewer/win32.c",
            "vncviewer/keysym2ucs.c",
        ])
        .compile("libtigervnc.a");
}

#[cfg(target_os = "macos")]
fn build_macos() {
    cc::Build::new()
        .includes(Path::new("common"))
        .includes(Some(Path::new("/usr/X11/include")))
        .flag("-DKEYBOARD_ONLY")
        .files(&["osx/VNCServer.m", "vncviewer/cocoa.mm"])
        .compile("libtigervnc.a");
}

#[cfg(target_os = "linux")]
fn build_linux() {
    cc::Build::new()
        .includes(Path::new("unix"))
        .includes(Path::new("common"))
        .flag("-DKEYBOARD_ONLY")
        .files(&[])
        .compile("libtigervnc.a");
}
