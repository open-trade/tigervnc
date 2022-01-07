extern "C" {
    fn unix_vkey_to_keysym(vkey: u16) -> i32;
    // https://tronche.com/gui/x/xlib/utilities/keyboard/XKeysymToString.html
    fn XKeysymToString(keysym: usize) -> *const u8;
}

pub fn vkey_to_keysym(vkey: u32) -> Option<u32> {
    let v = unsafe { unix_vkey_to_keysym(vkey as _) };
    if v <= 0 {
        None
    } else {
        Some(v as _)
    }
}

pub fn keysym_to_string(keysym: u32) -> Option<String> {
    let s = unsafe { XKeysymToString(keysym as _) };
    if s.is_null() {
        None
    } else {
        let s = unsafe { std::ffi::CStr::from_ptr(s as _) };
        Some(s.to_str().unwrap_or("").to_owned())
    }
}
