use crate::{
  c_char,
  char16_t,
  char32_t,
  mbstate_t,
  size_t,
  std::{errno, stdlib},
  support::locale
};

#[no_mangle]
pub extern "C" fn ouma_c16rtomb(
  s: *mut c_char,
  c16: char16_t,
  ps: *mut mbstate_t
) -> size_t {
  let loc = unsafe { *locale::get_thread_locale() };
  let mut buf: [c_char; stdlib::MB_LEN_MAX as usize] =
    [0; stdlib::MB_LEN_MAX as usize];
  let (s, c16) = if s.is_null() { (buf.as_mut_ptr(), 0) } else { (s, c16) };

  let c32: char32_t;
  let mut c16l: char16_t = 0;
  if locale::mbstate_get_surrogate(ps, &mut c16l) {
    if !(0xdc00..=0xdfff).contains(&c16) {
      errno::set_errno(errno::EILSEQ);
      return -1isize as usize;
    }
    c32 = 0x10000 + ((c16l & 0x3ff) << 10 | (c16 & 0x3ff)) as char32_t;
  } else if (0xd800..=0xdbff).contains(&c16) {
    locale::mbstate_set_surrogate(ps, c16);
    return 0;
  } else {
    c32 = c16 as char32_t;
  }

  let l = (loc.ctype.c32tomb)(s, c32, ps);
  if l >= 0 {
    locale::mbstate_set_init(ps);
  }
  l as size_t
}

#[no_mangle]
pub extern "C" fn ouma_c32rtomb(
  s: *mut c_char,
  c32: char32_t,
  ps: *mut mbstate_t
) -> size_t {
  let loc = unsafe { *locale::get_thread_locale() };
  let mut buf: [c_char; stdlib::MB_LEN_MAX as usize] =
    [0; stdlib::MB_LEN_MAX as usize];
  let (s, c32) = if s.is_null() { (buf.as_mut_ptr(), 0) } else { (s, c32) };
  let l = (loc.ctype.c32tomb)(s, c32, ps);
  if l >= 0 {
    locale::mbstate_set_init(ps);
  }
  l as size_t
}

#[no_mangle]
pub extern "C" fn ouma_mbrtoc16(
  pc16: *mut char16_t,
  s: *const c_char,
  n: size_t,
  ps: *mut mbstate_t
) -> size_t {
  let loc = unsafe { *locale::get_thread_locale() };
  let mut c16: char16_t = 0;
  let (pc16, s, n) = if s.is_null() {
    (core::ptr::addr_of_mut!(c16), core::ptr::null::<c_char>(), 1 as size_t)
  } else if pc16.is_null() {
    (core::ptr::addr_of_mut!(c16), s, n)
  } else {
    (pc16, s, n)
  };
  if locale::mbstate_get_surrogate(ps, pc16) {
    locale::mbstate_set_init(ps);
    return -3isize as usize;
  }
  if n == 0 {
    locale::mbstate_set_init(ps);
    return -2isize as usize;
  }
  let mut c32: char32_t = 0;
  let l = (loc.ctype.mbtoc32)(&mut c32, s, n, ps);
  if l >= 0 {
    if c32 < 0x10000 {
      unsafe { *pc16 = c32 as char16_t };
    } else {
      c32 -= 0x10000;
      unsafe { *pc16 = 0xd800 | (c32 >> 10) as char16_t };
      locale::mbstate_set_surrogate(ps, 0xdc00 | (c32 & 0x3ff) as char16_t);
    }
    unsafe {
      if *pc16 == 0 {
        return 0;
      }
    }
  }
  l as size_t
}

#[no_mangle]
pub extern "C" fn ouma_mbrtoc32(
  pc32: *mut char32_t,
  s: *const c_char,
  n: size_t,
  ps: *mut mbstate_t
) -> size_t {
  let loc = unsafe { *locale::get_thread_locale() };
  let mut c32: char32_t = 0;
  let (pc32, s, n) = if s.is_null() {
    (core::ptr::addr_of_mut!(c32), core::ptr::null::<c_char>(), 1 as size_t)
  } else if pc32.is_null() {
    (core::ptr::addr_of_mut!(c32), s, n)
  } else {
    (pc32, s, n)
  };
  let l = (loc.ctype.mbtoc32)(pc32, s, n, ps);
  unsafe {
    if l >= 0 && *pc32 == '\0' as char32_t {
      return 0;
    }
  }
  l as size_t
}
