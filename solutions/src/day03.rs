use itertools::Itertools;

use error::AocResult;

pub fn parse_input(fname: &str) -> AocResult<Vec<Vec<u8>>> {
  let b = std::fs::read(fname)?;
  Ok(b.split(|c| *c == b'\n').map(|bs| bs.to_vec()).collect_vec())
}

fn priority(n: u8) -> u8 {
  if n & 32 == 0 {
    // upper
    (n & 31) + 26
  } else {
    // lower
    n & 31
  }
}

fn part1(input: &[Vec<u8>]) -> AocResult<i32> {
  let mut counts = [0; 53];
  let mut res = 0;
  for s in input {
    counts.fill(0);

    let n = s.len();
    for c in &s[0..n / 2] {
      counts[priority(*c) as usize] += 1;
    }
    for c in &s[n / 2..n] {
      let p = priority(*c);
      if counts[p as usize] > 0 {
        res += p as i32;
        break; // because unique
      }
    }
  }
  Ok(res)
}

fn part2(input: &[Vec<u8>]) -> AocResult<i32> {
  let mut counts = [0u8; 53];
  let mut res = 0;
  for (c1, c2, c3) in input.into_iter().tuples() {
    counts.fill(0);

    for c in c1 {
      counts[priority(*c) as usize] |= 1;
    }
    for c in c2 {
      counts[priority(*c) as usize] |= 2;
    }
    for c in c3 {
      let p = priority(*c);
      if counts[p as usize] == 3 {
        res += p as i32;
        break; // because unique
      }
    }
  }
  Ok(res)
}

pub fn run(input: &[Vec<u8>]) -> AocResult<(i32, i32)> {
  Ok((part1(input)?, part2(input)?))
}
