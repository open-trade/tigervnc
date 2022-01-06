cfg_if::cfg_if! {
    if #[cfg(windows)] {
        mod win;
        pub use win::*;
    } else if #[cfg(target_os = "macos")] {
    } else if #[cfg(target_os = "linux")] {
    }
}
