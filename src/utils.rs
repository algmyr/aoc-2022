use crate::error::AocResult;

pub fn num_from_bytes<T: num::PrimInt>(bytes: &[u8]) -> T {
  let mut n = 0;
  for c in bytes {
    n = n * 10 + (c - b'0') as i32;
  }

  let res = unsafe { T::from(n).unwrap_unchecked() };
  res
}
pub fn read_all_nums_from_bytes<T: num::PrimInt>(bytes: &[u8]) -> AocResult<Vec<T>> {
  let mut res = vec![];

  let mut n = 0;
  let mut reading = false;
  for c in bytes {
    if reading {
      if c.is_ascii_digit() {
        n = n * 10 + (c - b'0') as i32;
      } else {
        res.push(unsafe { T::from(n).unwrap_unchecked() });
        reading = false;
      }
    } else {
      if c.is_ascii_digit() {
        n = (c - b'0') as i32;
        reading = true;
      }
    }
  }

  Ok(res)
}

pub fn read_all_nums<T: num::PrimInt>(fname: &str) -> AocResult<Vec<T>> {
  let contents = std::fs::read(fname)?;
  read_all_nums_from_bytes(&contents)
}

pub fn read_all_signed_nums<T: num::PrimInt>(fname: &str) -> AocResult<Vec<T>> {
  let contents = std::fs::read(fname)?;

  let mut res = vec![];

  let mut n = 0i32;
  let mut negative = false;
  let mut reading = false;
  for c in contents {
    if reading {
      if c.is_ascii_digit() {
        n = n * 10 + (c - b'0') as i32;
      } else {
        res.push(unsafe { T::from(if negative { -n } else { n }).unwrap_unchecked() });
        reading = false;
      }
    } else {
      if c.is_ascii_digit() {
        n = (c - b'0') as i32;
        negative = false;
        reading = true;
      } else if c == b'-' {
        n = 0;
        negative = true;
        reading = true;
      }
    }
  }

  Ok(res)
}
