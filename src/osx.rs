extern "C" {
    fn osx_vkey_to_keysym(vkey: u16) -> i32;
}

pub fn vkey_to_keysym(vkey: u32, _extended: i32) -> Option<u32> {
    let v = unsafe { osx_vkey_to_keysym(vkey as _) };
    if v < 0 {
        None
    } else {
        Some(v as _)
    }
}

pub fn keysym_to_vkey(_keysym: u32) -> Option<(u8, u8)> {
    None
}
