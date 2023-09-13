use {
  crate::{
    c_char,
    c_int,
    std::{ctype, errno}
  },
  num_traits::{cast, identities}
};

fn b36_char_to_int(input: c_char) -> c_int {
  if ctype::ouma_isdigit(c_int::from(input)) != 0 {
    return c_int::from(input - b'0' as c_char);
  }
  if ctype::ouma_isalpha(c_int::from(input)) != 0 {
    return c_int::from(
      (input | 32).wrapping_add(10).wrapping_sub(b'a' as c_char)
    );
  }
  0
}

fn is_hex_start(src: *const c_char) -> bool {
  unsafe {
    if *src == b'0' as c_char &&
      ((*(src.wrapping_add(1)) | 32) == b'x' as c_char ||
        (*(src.wrapping_add(1)) | 32) == b'X' as c_char) &&
      b36_char_to_int(*(src.wrapping_add(2))) < 16 &&
      ctype::ouma_isalnum(c_int::from(*(src.wrapping_add(2)))) != 0
    {
      return true;
    }
  }
  false
}

fn is_bin_start(src: *const c_char) -> bool {
  unsafe {
    if *src == b'0' as c_char &&
      ((*(src.wrapping_add(1)) | 32) == b'b' as c_char ||
        (*(src.wrapping_add(1)) | 32) == b'B' as c_char) &&
      b36_char_to_int(*(src.wrapping_add(2))) < 16 &&
      ctype::ouma_isalnum(c_int::from(*(src.wrapping_add(2)))) != 0
    {
      return true;
    }
  }
  false
}

fn infer_base(src: *mut *const c_char) -> c_int {
  unsafe {
    if is_hex_start(*src) {
      *src = (*src).wrapping_offset(2);
      return 16;
    } else if is_bin_start(*src) {
      *src = (*src).wrapping_offset(2);
      return 2;
    } else if **src == b'0' as c_char {
      return 8;
    }
  }
  10
}

// (result, parsed, err)
pub fn strtointeger<T, const MIN: u64, const MAX: u64>(
  src: *const c_char,
  base: c_int
) -> (T, isize, c_int)
where
  T: num_traits::Zero + cast::NumCast {
  let save = src;
  let (mut s, mut b, mut errno, mut is_number, mut result) =
    (src, base, 0, false, 0u64);

  if b < 0 || b == 1 || b > 36 {
    errno = errno::EINVAL;
    return (identities::zero(), 0, errno);
  }

  unsafe {
    while ctype::ouma_isspace(c_int::from(*s)) != 0 {
      s = s.wrapping_add(1);
    }
  }

  let mut result_sign: c_char = b'+' as c_char;
  unsafe {
    if *s == b'+' as c_char || *s == b'-' as c_char {
      result_sign = *s;
      s = s.wrapping_add(1);
    }
  }

  if b == 0 {
    b = infer_base(&mut s);
  } else if b == 16 && is_hex_start(s) {
    s = s.wrapping_add(2);
  }

  let is_unsigned = MIN == 0;
  let is_positive = result_sign == b'+' as c_char;
  let negative_max = if !is_unsigned { MAX + 1 } else { MAX };
  let abs_max = if is_positive { MAX } else { negative_max };
  let abs_max_div = abs_max / b as u64;
  unsafe {
    while ctype::ouma_isalnum(c_int::from(*s)) != 0 {
      let cur_digit = b36_char_to_int(*s) as u64;
      if cur_digit >= b as u64 {
        break;
      }

      is_number = true;
      s = s.wrapping_add(1);

      if result == abs_max {
        errno = errno::ERANGE;
        continue;
      }
      if result > abs_max_div {
        result = abs_max;
        errno = errno::ERANGE;
      } else {
        result *= b as u64;
      }
      if result > abs_max - cur_digit {
        result = abs_max;
        errno = errno::ERANGE;
      } else {
        result += cur_digit;
      }
    }
  }

  let len: isize = if is_number { unsafe { s.offset_from(save) } } else { 0 };

  if errno == errno::ERANGE {
    if is_positive || is_unsigned {
      return (cast::cast(MAX).unwrap(), len, errno);
    }
    return (cast::cast(MIN).unwrap(), len, errno);
  }

  if is_positive {
    (cast::cast(result).unwrap(), len, errno)
  } else {
    (cast::cast(result.wrapping_neg()).unwrap(), len, errno)
  }
}
