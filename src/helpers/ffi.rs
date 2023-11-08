pub unsafe fn set_u32_ptr(ptr: *mut u32, val: u32) {
    if !ptr.is_null() {
        *ptr = val;
    }
}

pub unsafe fn set_i32_ptr(ptr: *mut i32, val: i32) {
    if !ptr.is_null() {
        *ptr = val;
    }
}

pub unsafe fn set_i16_ptr(ptr: *mut i16, val: i16) {
    if !ptr.is_null() {
        *ptr = val;
    }
}

pub unsafe fn set_i64_ptr(ptr: *mut i64, val: i64) {
    if !ptr.is_null() {
        *ptr = val;
    }
}

pub unsafe fn set_u64_ptr(ptr: *mut u64, val: u64) {
    if !ptr.is_null() {
        *ptr = val;
    }
}

pub fn from_cstr_or_default(cstr: *const i8, cstr_length: u32, default: &str) -> String {
    if cstr.is_null() {
        return default.to_string();
    }

    unsafe { String::from_utf8_lossy(std::slice::from_raw_parts(cstr as *const u8, cstr_length as usize)).to_string() }
}

pub fn try_from_cstr(cstr: *const i8, cstr_length: u32) -> Option<String> {
    if cstr.is_null() {
        return None;
    }

    Some(unsafe { String::from_utf8_lossy(std::slice::from_raw_parts(cstr as *const u8, cstr_length as usize)).to_string() })
}

pub fn from_cstr(cstr: *const i8, cstr_length: u32) -> String {
    unsafe { String::from_utf8_lossy(std::slice::from_raw_parts(cstr as *const u8, cstr_length as usize)).to_string() }
}

pub unsafe fn from_cba<'a>(cba: *const u8, cba_length: u32) -> &'a [u8] {
    std::slice::from_raw_parts(cba, cba_length as usize)
}

pub unsafe fn set_byte_buf(ptr: *mut *mut u8, val: &[u8]) {
    if !ptr.is_null() {
        *ptr = libc::malloc(val.len()) as *mut u8;
        std::ptr::copy_nonoverlapping(val.as_ptr(), *ptr, val.len());
    }
}

pub unsafe fn set_cstr(ptr: *mut *mut u8, cstr_len: *mut u32, val: String) {
    if !ptr.is_null() {
        *ptr = libc::malloc(val.len()) as *mut u8;

        std::ptr::copy_nonoverlapping(val.as_ptr(), *ptr, val.len());
        set_u32_ptr(cstr_len, val.len() as u32);
    }
}

pub unsafe fn set_cstr_from_str(ptr: *mut *mut u8, cstr_len: *mut u32, val: &str) {
    if !ptr.is_null() {
        *ptr = libc::malloc(val.len()) as *mut u8;

        std::ptr::copy_nonoverlapping(val.as_ptr(), *ptr, val.len());
        set_u32_ptr(cstr_len, val.len() as u32);
    }
}

pub unsafe fn set_cstr_array(ptr: *mut *mut u8, cstr_len: *mut u32, val: &[String]) {
    if !ptr.is_null() {
        let joined = val.join(",");
        *ptr = libc::malloc(joined.len()) as *mut u8;

        std::ptr::copy_nonoverlapping(joined.as_ptr(), *ptr, joined.len());
        set_u32_ptr(cstr_len, joined.len() as u32);
    }
}

pub unsafe fn set_u32_buf(ptr: *mut *mut u32, val: &[u32]) {
    if !ptr.is_null() {
        let val_len = std::mem::size_of_val(val);
        *ptr = libc::malloc(val_len) as *mut u32;
        std::ptr::copy_nonoverlapping::<u32>(val.as_ptr(), *ptr, val.len());
    }
}

pub unsafe fn free_ptr(ptr: *mut *mut ::std::os::raw::c_void) {
    libc::free(*ptr);
    *ptr = std::ptr::null_mut::<libc::c_void>();
}
