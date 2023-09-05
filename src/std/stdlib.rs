use {
  crate::{c_int, max_align_t, size_t, std::errno, support::locale},
  allocator::alloc,
  core::{ffi::c_void, mem, ptr}
};

pub const MB_LEN_MAX: c_int = 16;

#[no_mangle]
pub extern "C" fn __oumalibc_get_mb_cur_max() -> size_t {
  let loc = unsafe { *locale::get_thread_locale() };
  loc.ctype.mb_cur_max as size_t
}

#[no_mangle]
pub extern "C" fn ouma_malloc(size: size_t) -> *mut c_void {
  let size = if size == 0 { size + 1 } else { size };
  let layout = match alloc::Layout::from_size_align(
    size,
    mem::align_of::<max_align_t>()
  ) {
    | Err(_) => {
      errno::set_errno(errno::ENOMEM);
      return ptr::null_mut();
    },
    | Ok(result) => result
  };
  let ptr = unsafe { alloc::alloc(layout) };
  if ptr.is_null() {
    errno::set_errno(errno::ENOMEM);
    return ptr::null_mut();
  }

  ptr.cast()
}

#[no_mangle]
pub extern "C" fn ouma_aligned_alloc(
  alignment: size_t,
  size: size_t
) -> *mut c_void {
  let size = if size == 0 { size + 1 } else { size };
  let layout = match alloc::Layout::from_size_align(size, alignment) {
    | Err(_) => {
      errno::set_errno(errno::ENOMEM);
      return ptr::null_mut();
    },
    | Ok(result) => result
  };
  let ptr = unsafe { alloc::alloc(layout) };
  if ptr.is_null() {
    errno::set_errno(errno::ENOMEM);
    return ptr::null_mut();
  }

  ptr.cast()
}

#[no_mangle]
pub extern "C" fn ouma_calloc(
  nmemb: size_t,
  size: size_t
) -> *mut c_void {
  let Some(mut res) = nmemb.checked_mul(size) else {
    errno::set_errno(errno::ENOMEM);
    return ptr::null_mut();
  };
  if res == 0 {
    res += 1;
  }

  let layout =
    match alloc::Layout::from_size_align(res, mem::align_of::<max_align_t>()) {
      | Err(_) => {
        errno::set_errno(errno::ENOMEM);
        return ptr::null_mut();
      },
      | Ok(result) => result
    };
  let ptr = unsafe { alloc::alloc_zeroed(layout) };
  if ptr.is_null() {
    errno::set_errno(errno::ENOMEM);
    return ptr::null_mut();
  }

  ptr.cast()
}

#[no_mangle]
pub extern "C" fn ouma_realloc(
  ptr: *mut c_void,
  size: size_t
) -> *mut c_void {
  if ptr.is_null() {
    return ouma_malloc(size);
  }

  let layout = match alloc::Layout::from_size_align(
    size,
    mem::align_of::<max_align_t>()
  ) {
    | Err(_) => {
      errno::set_errno(errno::ENOMEM);
      return ptr::null_mut();
    },
    | Ok(result) => result
  };

  unsafe { alloc::realloc(ptr.cast(), layout, size).cast() }
}

// Inspired by mustang crate:
// https://github.com/sunfishcode/mustang/blob/main/c-scape/src/malloc/mod.rs
//
#[no_mangle]
pub extern "C" fn ouma_posix_memalign(
  memptr: *mut *mut c_void,
  alignment: size_t,
  size: size_t
) -> c_int {
  if !(alignment.is_power_of_two() &&
    alignment % mem::size_of::<*const c_void>() == 0)
  {
    return errno::EINVAL;
  }

  let ptr = ouma_aligned_alloc(alignment, size);
  unsafe {
    *memptr = ptr;
  }
  0
}

#[no_mangle]
pub extern "C" fn ouma_free(ptr: *mut c_void) {
  if ptr.is_null() {
    return;
  }

  let layout = alloc::Layout::new::<*mut c_void>();
  unsafe { alloc::dealloc(ptr.cast(), layout) };
}

#[no_mangle]
pub extern "C" fn ouma_free_sized(
  ptr: *mut c_void,
  size: size_t
) {
  if ptr.is_null() {
    return;
  }

  let layout = match alloc::Layout::from_size_align(
    size,
    mem::align_of::<max_align_t>()
  ) {
    | Err(_) => panic!("Cannot create memory layout"),
    | Ok(result) => result
  };
  unsafe { alloc::dealloc(ptr.cast(), layout) };
}

#[no_mangle]
pub extern "C" fn free_aligned_sized(
  ptr: *mut c_void,
  alignment: size_t,
  size: size_t
) {
  if ptr.is_null() {
    return;
  }

  let layout = match alloc::Layout::from_size_align(size, alignment) {
    | Err(_) => panic!("Cannot create memory layout"),
    | Ok(result) => result
  };
  unsafe { alloc::dealloc(ptr.cast(), layout) };
}
