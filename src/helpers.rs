pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

macro_rules! ok_or_return {
  ($wrapped:expr, $return:expr) => {
    match $wrapped {
      Ok(ok) => ok,
      Err(err) => return $return(err)
    }
  };
}

macro_rules! some_or_return {
  ($wrapped:expr, $return:expr) => {
    match $wrapped {
      Some(some) => some,
      None => return $return
    }
  };
}

macro_rules! try_or_return {
  ($wrapped:expr, $return:expr) => {
    match $wrapped() {
      Ok(ok) => ok,
      Err(err) => return $return(err)
    }
  };
}

#[macro_use]
pub mod rest {
    macro_rules! call {
        ($response:ident) => {{
            if !$response.status().is_success() {
                return Err(Error::Unsuccessful($response.status()).into());
            }

            Ok($response.json()?)
        }};
    }

    macro_rules! post {
        ($url:expr, $body:expr) => {{
            let response = CLIENT.post($url).json($body).send().map_err(|source| Error::RequestFailed { source })?;

            call!(response)
        }};
    }

    macro_rules! get {
        ($url:expr) => {{
            let response = CLIENT.get($url).send().map_err(|source| Error::RequestFailed { source })?;

            call!(response)
        }};
    }
}

pub mod ffi {
  pub fn set_u32_ptr(ptr: *mut u32, val: u32) {
      if !ptr.is_null() {
          unsafe { *ptr = val };
      }
  }

  pub fn set_i32_ptr(ptr: *mut i32, val: i32) {
      if !ptr.is_null() {
          unsafe { *ptr = val };
      }
  }

  pub fn set_i16_ptr(ptr: *mut i16, val: i16) {
      if !ptr.is_null() {
          unsafe { *ptr = val };
      }
  }

  pub fn set_i64_ptr(ptr: *mut i64, val: i64) {
      if !ptr.is_null() {
          unsafe { *ptr = val };
      }
  }

  pub fn set_u64_ptr(ptr: *mut u64, val: u64) {
      if !ptr.is_null() {
          unsafe { *ptr = val };
      }
  }

  pub fn from_cstr_or_default(cstr: *const i8, cstr_length: u32, default: &str) -> String {
      if cstr.is_null() {
          return default.to_string();
      }

      unsafe {
          String::from_utf8_lossy(std::slice::from_raw_parts(
              cstr as *const u8,
              cstr_length as usize,
          ))
          .to_string()
      }
  }

  pub fn try_from_cstr(cstr: *const i8, cstr_length: u32) -> Option<String> {
      if cstr.is_null() {
          return None;
      }

      Some(unsafe {
          String::from_utf8_lossy(std::slice::from_raw_parts(
              cstr as *const u8,
              cstr_length as usize,
          ))
          .to_string()
      })
  }

  pub fn from_cstr(cstr: *const i8, cstr_length: u32) -> String {
      unsafe {
          String::from_utf8_lossy(std::slice::from_raw_parts(
              cstr as *const u8,
              cstr_length as usize,
          ))
          .to_string()
      }
  }

  pub fn from_cba<'a>(cba: *const u8, cba_length: u32) -> &'a [u8] {
      unsafe { std::slice::from_raw_parts(cba, cba_length as usize) }
  }

  pub fn set_byte_buf(ptr: *mut *mut u8, val: &[u8]) {
      if !ptr.is_null() {
          unsafe {
              *ptr = libc::malloc(val.len()) as *mut u8;
              std::ptr::copy_nonoverlapping(val.as_ptr(), *ptr, val.len());
          }
      }
  }

  pub fn set_cstr(ptr: *mut *mut u8, cstr_len: *mut u32, val: String) {
      if !ptr.is_null() {
          unsafe {
              *ptr = libc::malloc(val.len()) as *mut u8;

              std::ptr::copy_nonoverlapping(val.as_ptr(), *ptr as *mut u8, val.len());
              set_u32_ptr(cstr_len, val.len() as u32);
          }
      }
  }

  pub fn set_u32_buf(ptr: *mut *mut u32, val: &[u32]) {
      if !ptr.is_null() {
          unsafe {
              let val_len = val.len() * std::mem::size_of::<u32>();
              *ptr = libc::malloc(val_len) as *mut u32;
              std::ptr::copy_nonoverlapping::<u32>(val.as_ptr(), *ptr, val.len());
          }
      }
  }

  pub fn free_ptr(ptr: *mut *mut ::std::os::raw::c_void) {
      unsafe {
          libc::free(*ptr);
          *ptr = std::ptr::null_mut::<libc::c_void>();
      }
  }
}
