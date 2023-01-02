use bstr::ByteSlice;
use itertools::Itertools;

use error::AocResult;

pub fn parse_input(fname: &str) -> AocResult<Vec<Vec<u8>>> {
  let b = std::fs::read(fname)?;
  let mut res = b.split(|c| *c == b'\n').map(|bs| bs.to_vec()).collect_vec();
  if let Some(b) = res.last() {
    if b.is_empty() {
      res.pop();
    }
  }
  Ok(res)
}

fn solve(input: &[Vec<u8>]) -> AocResult<(i32, i32)> {
  let mut stack = vec![];
  let mut sizes = vec![];
  let mut total = 0;
  for line in input {
    // This is hacky...
    if line[0] == b'd' {
      // dir *
    } else if line[2] == b'l' {
      // $ ls
    } else if line.get(6) == Some(&b'.') {
      // $ cd ..
      let size = stack.pop().unwrap();
      sizes.push(size);
      *stack.last_mut().unwrap() += size;
    } else if line[2] == b'c' {
      // $ cd *
      stack.push(0);
    } else {
      // 123 file.ext
      let (size, _) = line.split_once_str(b" ").unwrap();
      let size: i32 = crate::utils::num_from_bytes(size);
      total += size;
      *stack.last_mut().unwrap() += size;
    }
  }
  while let Some(size) = stack.pop() {
    sizes.push(size);
    if let Some(last) = stack.last_mut() {
      *last += size;
    }
  }

  let res1 = sizes.iter().filter(|x| **x <= 100_000).sum();
  let target = 30_000_000 - (70_000_000 - total);
  let res2 = sizes
    .into_iter()
    .filter(|x| *x >= target)
    .min()
    .expect("no min?");
  Ok((res1, res2))
}

pub fn run(input: &[Vec<u8>]) -> AocResult<(i32, i32)> { solve(input) }
