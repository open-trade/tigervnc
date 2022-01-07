cfg_if::cfg_if! {
    if #[cfg(windows)] {
        mod win;
        pub use win::*;
    } else if #[cfg(target_os = "macos")] {
        mod osx;
        pub use osx::*;
    } else if #[cfg(unix)] {
        mod unix;
        pub use unix::*;
    }
}
