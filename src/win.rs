extern "C" {
    fn win32_vkey_to_keysym(vkey: u32, extended: i32) -> i32;
    fn win32_keysym_to_vkey(keysym: u32, vkCode: *mut u8, extended: *mut u8) -> i32;
}

pub fn vkey_to_keysym(vkey: u32, extended: i32) -> Option<u32> {
    let v = unsafe { win32_vkey_to_keysym(vkey, extended) };
    if v < 0 {
        None
    } else {
        Some(v as _)
    }
}

pub fn keysym_to_vkey(keysym: u32) -> Option<(u8, u8)> {
    let mut vkey: u8 = 0;
    let mut extended: u8 = 0;
    if unsafe { win32_keysym_to_vkey(keysym, &mut vkey, &mut extended) } > 0 {
        Some((vkey, extended))
    } else {
        None
    }
}