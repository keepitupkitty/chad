pub mod ascii;
pub mod utf8;

use {
  crate::{c_char, c_int, char32_t, mbstate_t, size_t, ssize_t},
  core::ptr
};

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct LocaleCtype {
  pub codeset: *const c_char,
  pub mbtoc32:
    fn(*mut char32_t, *const c_char, size_t, *mut mbstate_t) -> ssize_t,
  pub c32tomb: fn(*mut c_char, char32_t, *mut mbstate_t) -> ssize_t,
  pub mb_cur_max: c_int
}

impl LocaleCtype {
  pub fn new() -> Self {
    Self {
      codeset: ptr::null::<c_char>(),
      mbtoc32: |_, _, _, _| unimplemented!(),
      c32tomb: |_, _, _| unimplemented!(),
      mb_cur_max: 0
    }
  }

  pub fn as_ptr(&mut self) -> *mut LocaleCtype {
    &mut *self
  }
}

impl Default for LocaleCtype {
  fn default() -> Self {
    Self::new()
  }
}
