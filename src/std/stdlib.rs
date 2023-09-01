use crate::{c_int, size_t, support::locale};

pub const MB_LEN_MAX: c_int = 16;

#[no_mangle]
pub extern "C" fn __oumalibc_get_mb_cur_max() -> size_t {
  let loc = unsafe { *locale::get_thread_locale() };
  loc.ctype.mb_cur_max as size_t
}
