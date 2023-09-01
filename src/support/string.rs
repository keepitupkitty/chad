use {
  crate::{
    c_char,
    c_int,
    size_t,
    std::signal,
    support::stringstream::StringStream,
    wchar_t
  },
  core::{fmt, slice}
};

#[inline]
pub fn string_length(string: *const c_char) -> size_t {
  let mut len: size_t = 0;
  let mut s = string;
  unsafe {
    while *s != 0 {
      s = s.wrapping_offset(1);
      len += 1;
    }
  }
  len
}

#[inline]
pub fn wstring_length(string: *const wchar_t) -> size_t {
  let mut len: size_t = 0;
  let mut s = string;
  unsafe {
    while *s != 0 {
      s = s.wrapping_offset(1);
      len += 1;
    }
  }
  len
}

#[inline]
pub fn build_signal_string(
  num: c_int,
  buf: *mut c_char,
  len: usize
) -> *const c_char {
  let mut n = num;
  let prefix = if (signal::SIGRTMIN..=signal::SIGRTMAX).contains(&n) {
    n -= signal::SIGRTMIN;
    "Real-time"
  } else {
    "Unknown"
  };

  let mut ss =
    unsafe { StringStream::new(slice::from_raw_parts_mut(buf, len)) };
  fmt::write(&mut ss, format_args!("{prefix} signal {n}\0")).expect(
    "Error occurred while trying to write in stream, is buffer too short?"
  );
  buf
}

#[inline]
pub fn build_error_string(
  num: c_int,
  buf: *mut c_char,
  len: usize
) -> *const c_char {
  let mut ss =
    unsafe { StringStream::new(slice::from_raw_parts_mut(buf, len)) };
  fmt::write(&mut ss, format_args!("Unknown error {num}\0")).expect(
    "Error occurred while trying to write in stream, is buffer too short?"
  );
  buf
}
