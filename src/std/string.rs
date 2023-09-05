use {
  crate::{
    c_char,
    c_int,
    c_uchar,
    locale_t,
    size_t,
    std::{errno, signal, stdlib},
    support::{string, string::string_length, stringstream::StringStream}
  },
  cbitset::BitSet256,
  core::{arch::asm, ffi::c_void, fmt, ptr, slice}
};

#[no_mangle]
pub extern "C" fn ouma_memccpy(
  dest: *mut c_void,
  src: *const c_void,
  c: c_int,
  n: size_t
) -> *mut c_void {
  let mut dest1: *mut c_uchar = dest.cast::<c_uchar>();
  let mut src1: *const c_uchar = src.cast::<c_uchar>();
  let mut i = n;
  while i != 0 {
    let s = src1;
    src1 = src1.wrapping_offset(1);
    let d = dest1;
    dest1 = dest1.wrapping_offset(1);
    unsafe {
      *d = *s;
      if *d == c as c_uchar {
        return dest1.cast::<c_void>();
      }
    }
    i -= 1;
  }
  ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn ouma_memchr(
  s: *const c_void,
  c: c_int,
  n: size_t
) -> *mut c_void {
  let mut s1: *const c_uchar = s.cast::<c_uchar>();
  let mut i = n;
  while i != 0 {
    unsafe {
      if *s1 == c as c_uchar {
        return s1 as *mut c_void;
      }
    }
    s1 = s1.wrapping_offset(1);
    i -= 1;
  }
  ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn ouma_memcmp(
  left: *const c_void,
  right: *const c_void,
  n: size_t
) -> c_int {
  let l = left.cast::<c_uchar>();
  let r = right.cast::<c_uchar>();
  let mut i = 0;
  while i < n {
    let a = unsafe { *l.wrapping_add(i) };
    let b = unsafe { *r.wrapping_add(i) };
    if a != b {
      return a as c_int - b as c_int;
    }
    i += 1;
  }
  0
}

#[no_mangle]
pub extern "C" fn ouma_memcpy(
  dest: *mut c_void,
  src: *const c_void,
  n: size_t
) -> *mut c_void {
  let mut dest1: *mut c_uchar = dest.cast::<c_uchar>();
  let mut src1: *const c_uchar = src.cast::<c_uchar>();
  let mut i = 0;
  while i < n {
    let s = src1;
    src1 = src1.wrapping_offset(1);
    let d = dest1;
    dest1 = dest1.wrapping_offset(1);
    unsafe { *d = *s };
    i += 1;
  }
  dest
}

#[no_mangle]
pub extern "C" fn ouma_memmove(
  dest: *mut c_void,
  src: *const c_void,
  n: size_t
) -> *mut c_void {
  let mut dest1: *mut c_uchar = dest.cast::<c_uchar>();
  let mut src1: *const c_uchar = src.cast::<c_uchar>();
  if dest1.cast_const() < src1 {
    let mut i = 0;
    while i < n {
      let s = src1;
      src1 = src1.wrapping_offset(1);
      let d = dest1;
      dest1 = dest1.wrapping_offset(1);
      unsafe { *d = *s };
      i += 1;
    }
  } else if dest1.cast_const() > src1 {
    let mut i = n;
    src1 = src1.wrapping_add(i);
    dest1 = dest1.wrapping_add(i);

    while i != 0 {
      i -= 1;
      src1 = src1.wrapping_offset(-1);
      dest1 = dest1.wrapping_offset(-1);
      unsafe { *dest1 = *src1 };
    }
  }
  dest
}

#[no_mangle]
pub extern "C" fn ouma_memset(
  s: *mut c_void,
  c: c_int,
  n: size_t
) -> *mut c_void {
  let mut s1: *mut c_char = s.cast::<c_char>();
  let mut i = 0;
  while i < n {
    let s2 = s1;
    s1 = s1.wrapping_offset(1);
    unsafe {
      *s2 = c as c_char;
    }
    i += 1;
  }
  s
}

#[no_mangle]
pub extern "C" fn ouma_memset_explicit(
  s: *mut c_void,
  c: c_int,
  n: size_t
) -> *mut c_void {
  let s = ouma_memset(s, c, n);
  unsafe { asm!("/* {0} */", inout(reg) s => _) };
  s
}

#[no_mangle]
pub extern "C" fn ouma_strchr(
  s: *const c_char,
  c: c_int
) -> *mut c_char {
  let mut s1 = s;
  loop {
    unsafe {
      if *s1 == c as c_char {
        return s1.cast_mut();
      }
      if *s1 == 0 {
        return ptr::null_mut();
      }
    }
    s1 = s1.wrapping_offset(1);
  }
}

#[no_mangle]
pub extern "C" fn ouma_strrchr(
  s: *const c_char,
  c: c_int
) -> *mut c_char {
  let mut s1 = s;
  let mut last = ptr::null_mut();
  loop {
    unsafe {
      if *s1 == c as c_char {
        last = s1.cast_mut();
      }
      if *s1 == 0 {
        return last;
      }
    }
    s1 = s1.wrapping_offset(1);
  }
}

#[no_mangle]
pub extern "C" fn ouma_stpcpy(
  dest: *mut c_char,
  src: *const c_char
) -> *mut c_char {
  let len = string_length(src) + 1;
  ouma_stpncpy(dest, src, len)
}

#[no_mangle]
pub extern "C" fn ouma_stpncpy(
  dest: *mut c_char,
  src: *const c_char,
  n: size_t
) -> *mut c_char {
  let mut i = n;
  let mut d = dest;
  let mut s = src;
  unsafe {
    while i > 0 && *s != 0 {
      let d1 = d;
      d = d.wrapping_offset(1);
      let s1 = s;
      s = s.wrapping_offset(1);
      *d1 = *s1;
      i = i.wrapping_sub(1);
    }
  }
  let end = d;
  while i > 0 {
    let d1 = d;
    d = d.wrapping_offset(1);
    unsafe {
      *d1 = 0;
    }
    i -= 1;
  }
  end
}

#[no_mangle]
pub extern "C" fn ouma_strcat(
  dest: *mut c_char,
  src: *const c_char
) -> *mut c_char {
  let len = string_length(src) + 1;
  ouma_strncat(dest, src, len)
}

#[no_mangle]
pub extern "C" fn ouma_strncat(
  dest: *mut c_char,
  src: *const c_char,
  n: size_t
) -> *mut c_char {
  let mut i = n;

  if n != 0 {
    let mut d = dest;
    let mut s = src;
    unsafe {
      while *d != 0 {
        d = d.wrapping_offset(1);
      }
    }
    while i != 0 {
      let s1 = s;
      s = s.wrapping_offset(1);
      unsafe {
        *d = *s1;
        if *d == 0 {
          break;
        }
      }
      d = d.wrapping_offset(1);
      i -= 1;
    }
    unsafe { *d = 0 };
  }
  dest
}

#[no_mangle]
pub extern "C" fn ouma_strcmp(
  left: *const c_char,
  right: *const c_char
) -> c_int {
  let len = string_length(right) + 1;
  ouma_strncmp(left, right, len)
}

#[no_mangle]
pub extern "C" fn ouma_strncmp(
  left: *const c_char,
  right: *const c_char,
  n: size_t
) -> c_int {
  let mut l = left;
  let mut r = right;
  let mut i = n;
  while i != 0 {
    let l2 = l;
    l = l.wrapping_offset(1);
    let c1 = unsafe { *l2 as c_uchar };
    let r2 = r;
    r = r.wrapping_offset(1);
    let c2 = unsafe { *r2 as c_uchar };
    if c1 != c2 {
      return c1 as c_int - c2 as c_int;
    }
    if c1 == 0 {
      break;
    }
    i -= 1;
  }
  0
}

#[no_mangle]
pub extern "C" fn ouma_strcpy(
  dest: *mut c_char,
  src: *const c_char
) -> *mut c_char {
  let len = string_length(src) + 1;
  ouma_strncpy(dest, src, len)
}

#[no_mangle]
pub extern "C" fn ouma_strncpy(
  dest: *mut c_char,
  src: *const c_char,
  n: size_t
) -> *mut c_char {
  let mut i = n;
  let mut d = dest;
  let mut s = src;
  unsafe {
    while i > 0 && *s != 0 {
      let d1 = d;
      d = d.wrapping_offset(1);
      let s1 = s;
      s = s.wrapping_offset(1);
      *d1 = *s1;
      i = i.wrapping_sub(1);
    }
  }
  while i != 0 {
    let d1 = d;
    d = d.wrapping_offset(1);
    unsafe {
      *d1 = 0;
    }
    i -= 1;
  }
  dest
}

#[no_mangle]
pub extern "C" fn ouma_strlen(s: *const c_char) -> size_t {
  string_length(s)
}

#[no_mangle]
pub extern "C" fn ouma_strnlen(
  s: *const c_char,
  n: size_t
) -> size_t {
  let mut i = 0;
  while i < n {
    unsafe {
      if *s.wrapping_add(i) == 0 {
        break;
      }
    }
    i += 1;
  }
  i as size_t
}

#[no_mangle]
pub extern "C" fn ouma_strcspn(
  src: *const c_char,
  segment: *const c_char
) -> size_t {
  let mut s1 = src;
  let mut s2 = segment;
  let mut bitset = BitSet256::new();
  let mut i = 0;
  unsafe {
    while *s2 != 0 {
      bitset.insert(*s2 as usize);
      s2 = s2.wrapping_offset(1);
    }
    while *s1 != 0 && !bitset.contains(*s1 as usize) {
      i += 1;
      s1 = s1.wrapping_offset(1);
    }
  }
  i
}

#[no_mangle]
pub extern "C" fn ouma_strspn(
  src: *const c_char,
  segment: *const c_char
) -> size_t {
  let mut s1 = src;
  let mut s2 = segment;
  let mut bitset = BitSet256::new();
  let mut i = 0;
  unsafe {
    while *s2 != 0 {
      bitset.insert(*s2 as usize);
      s2 = s2.wrapping_offset(1);
    }
    while *s1 != 0 && bitset.contains(*s1 as usize) {
      i += 1;
      s1 = s1.wrapping_offset(1);
    }
  }
  i
}

#[no_mangle]
pub extern "C" fn ouma_strpbrk(
  src: *const c_char,
  breakset: *const c_char
) -> *mut c_char {
  let mut s1 = src;
  let mut s2 = breakset;
  let mut bitset = BitSet256::new();
  unsafe {
    while *s2 != 0 {
      bitset.insert(*s2 as usize);
      s2 = s2.wrapping_offset(1);
    }
    while *s1 != 0 && !bitset.contains(*s1 as usize) {
      s1 = s1.wrapping_offset(1);
    }
    if *s1 != 0 { s1.cast_mut() } else { ptr::null_mut() }
  }
}

#[no_mangle]
pub extern "C" fn ouma_strstr(
  haystack: *const c_char,
  needle: *const c_char
) -> *mut c_char {
  let mut h = haystack;
  let len = string_length(needle);
  if len == 0 {
    return h.cast_mut();
  }
  unsafe {
    while *h.wrapping_offset(0_isize) != 0 {
      let mut i = 0;
      loop {
        if *needle.wrapping_offset(i as isize) == 0 {
          return h.cast_mut();
        }
        if *h.wrapping_offset(i as isize) != *needle.wrapping_offset(i as isize)
        {
          break;
        }
        i += 1;
      }
      h = h.wrapping_offset(1);
    }
  }
  ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn ouma_strtok(
  s: *mut c_char,
  sep: *const c_char
) -> *mut c_char {
  static mut LAST: *mut c_char = ptr::null_mut();
  unsafe { ouma_strtok_r(s, sep, &mut LAST) }
}

#[no_mangle]
pub extern "C" fn ouma_strtok_r(
  s: *mut c_char,
  sep: *const c_char,
  lasts: *mut *mut c_char
) -> *mut c_char {
  let mut s1 = s;
  let mut sep1 = sep;
  let mut bitset = BitSet256::new();
  if s1.is_null() {
    s1 = unsafe { *lasts };
    if s1.is_null() {
      return ptr::null_mut();
    }
  }
  unsafe {
    while *sep1 != 0 {
      bitset.insert(*sep1 as usize);
      sep1 = sep1.wrapping_offset(1);
    }
    while *s1 != 0 && bitset.contains(*s1 as usize) {
      s1 = s1.wrapping_offset(1);
    }
    if *s1 == 0 {
      *lasts = s1;
      return ptr::null_mut();
    }
  }
  let token = s1;
  unsafe {
    while *s1 != 0 {
      if bitset.contains(*s1 as usize) {
        *s1 = 0;
        s1 = s1.wrapping_offset(1);
        break;
      }
      s1 = s1.wrapping_offset(1);
    }
    *lasts = s1;
  }
  token
}

#[no_mangle]
pub extern "C" fn ouma_strcoll(
  s1: *const c_char,
  s2: *const c_char
) -> c_int {
  ouma_strcmp(s1, s2)
}

#[no_mangle]
pub extern "C" fn ouma_strcoll_l(
  s1: *const c_char,
  s2: *const c_char,
  _: locale_t
) -> c_int {
  ouma_strcmp(s1, s2)
}

#[no_mangle]
pub extern "C" fn ouma_strxfrm(
  s1: *mut c_char,
  s2: *const c_char,
  n: size_t
) -> size_t {
  let len = string_length(s2);
  if len < n {
    ouma_strncpy(s1, s2, n);
  }
  len
}

#[no_mangle]
pub extern "C" fn ouma_strxfrm_l(
  s1: *mut c_char,
  s2: *const c_char,
  n: size_t,
  _: locale_t
) -> size_t {
  let len = string_length(s2);
  if len < n {
    ouma_strncpy(s1, s2, n);
  }
  len
}

#[no_mangle]
pub extern "C" fn ouma_strndup(
  s: *const c_char,
  sz: size_t
) -> *mut c_char {
  let len = ouma_strnlen(s, sz);
  let c: *mut c_char = stdlib::ouma_malloc(len + 1).cast::<c_char>();
  if c.is_null() {
    return ptr::null_mut();
  }
  ouma_memcpy(c.cast::<c_void>(), s.cast::<c_void>(), len);
  unsafe {
    *c.wrapping_add(len) = b'\0' as c_char;
  }
  c
}

#[no_mangle]
pub extern "C" fn ouma_strdup(s: *const c_char) -> *mut c_char {
  let len = string_length(s) + 1;
  let c: *mut c_char = stdlib::ouma_malloc(len).cast::<c_char>();
  if c.is_null() {
    return ptr::null_mut();
  }
  ouma_memcpy(c.cast::<c_void>(), s.cast::<c_void>(), len);
  c
}

#[thread_local]
static mut errbuf: [u8; 255] = [0; 255];

#[no_mangle]
pub extern "C" fn ouma_strerror_r(
  num: c_int,
  buf: *mut c_char,
  len: size_t
) -> c_int {
  if 0 <= num && (num as usize) < errno::SYS_ERRLIST.len() {
    let errstr = errno::SYS_ERRLIST.get(num as usize).unwrap();
    if (errstr.len() + 1 > len) || buf.is_null() {
      return errno::ERANGE;
    }
    let mut ss =
      unsafe { StringStream::new(slice::from_raw_parts_mut(buf, len)) };
    fmt::write(&mut ss, format_args!("{errstr}\0")).expect(
      "Error occurred while trying to write in stream, is buffer too short?"
    );
  } else {
    string::build_error_string(num, buf, len);
    return errno::EINVAL;
  }
  0
}

#[no_mangle]
pub extern "C" fn ouma_strerror(num: c_int) -> *mut c_char {
  unsafe {
    if ouma_strerror_r(num, errbuf.as_mut_ptr().cast(), errbuf.len()) != 0 {
      errno::set_errno(errno::EINVAL);
    }
    errbuf.as_mut_ptr().cast()
  }
}

#[no_mangle]
pub extern "C" fn ouma_strerror_l(
  num: c_int,
  _: locale_t
) -> *mut c_char {
  ouma_strerror(num)
}

#[thread_local]
static mut sigbuf: [u8; 255] = [0; 255];

#[no_mangle]
pub extern "C" fn ouma_strsignal(num: c_int) -> *mut c_char {
  if 0 <= num && (num as usize) < signal::SYS_SIGLIST.len() {
    let sigstr = signal::SYS_SIGLIST.get(num as usize).unwrap();
    unsafe {
      let mut ss = StringStream::new(slice::from_raw_parts_mut(
        sigbuf.as_mut_ptr().cast(),
        sigbuf.len()
      ));

      fmt::write(&mut ss, format_args!("{sigstr}\0")).expect(
        "Error occurred while trying to write in stream, is buffer too short?"
      );
      sigbuf.as_mut_ptr().cast()
    }
  } else {
    unsafe {
      string::build_signal_string(num, sigbuf.as_mut_ptr().cast(), sigbuf.len())
        .cast_mut()
    }
  }
}
