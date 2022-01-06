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
        .includes(std::path::Path::new("win"))
        .includes(std::path::Path::new("common"))
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
        .includes(std::path::Path::new("common"))
        .flag("-DKEYBOARD_ONLY")
        .files(&["vncviewer/cocoa.mm"])
        .compile("libtigervnc.a");
}

#[cfg(target_os = "linux")]
fn build_linux() {
    cc::Build::new()
        .includes(std::path::Path::new("unix"))
        .includes(std::path::Path::new("common"))
        .flag("-DKEYBOARD_ONLY")
        .files(&[])
        .compile("libtigervnc.a");
}
