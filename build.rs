fn main() {
    #[cfg(windows)]
    build_windows();
}

#[cfg(windows)]
fn build_windows() {
    cc::Build::new()
        .includes(std::path::Path::new("win"))
        .includes(std::path::Path::new("common"))
        .flag("-DKEYBOARD_ONLY")
        .files(&["win/rfb_win32/SInput.cxx", "vncviewer/win32.c"])
        .compile("libtigervnc.a");
}
