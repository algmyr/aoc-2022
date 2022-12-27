use std::ops::Add;

use bstr::ByteSlice;
use itertools::Itertools;

use crate::error::AocResult;

pub fn parse_input(fname: &str) -> AocResult<Vec<Vec<u8>>> {
  let b = std::fs::read(fname)?;
  Ok(b.split(|c| *c == b'\n').map(|bs| bs.to_vec()).collect_vec())
}

struct BalancedQuinary {
  digits: Vec<i32>,
}

impl BalancedQuinary {
  fn new(bs: &[u8]) -> Self {
    let mut digits = vec![];
    for b in bs.iter().rev() {
      digits.push(match b {
        b'=' => -2,
        b'-' => -1,
        b'0' => 0,
        b'1' => 1,
        b'2' => 2,
        _ => panic!(":<"),
      });
    }
    Self { digits }
  }

  fn to_str(&self) -> String {
    let mut bvec = vec![];
    for d in self.digits.iter().rev() {
      bvec.push(match d {
        -2 => b'=',
        -1 => b'-',
        0 => b'0',
        1 => b'1',
        2 => b'2',
        _ => panic!(":<"),
      });
    }
    bvec.to_str_lossy().to_string()
  }
}

impl Add for BalancedQuinary {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    use itertools::EitherOrBoth::*;

    let mut carry = 0;
    let mut digits = vec![];
    for pair in self.digits.into_iter().zip_longest(other.digits.into_iter()) {
      let mut sum = match pair {
        Both(l, r) => l+r,
        Left(l) => l,
        Right(r) => r,
      } + carry;
      if sum < -2 {
        carry = -1;
        sum += 5;
      } else if sum > 2 {
        carry = 1;
        sum -= 5;
      } else {
        carry = 0;
      }
      digits.push(sum);
    }
    if carry != 0 {
      digits.push(carry);
    }

    Self { digits }
  }
}

fn part1(input: &[Vec<u8>]) -> AocResult<String> {
  let res = input.iter().map(|x|
    BalancedQuinary::new(&*x)
  ).fold(
    BalancedQuinary::new(b"0"), |accu, bq| {
      accu + bq
    });
  Ok(res.to_str())
}

fn part2(_input: &[Vec<u8>]) -> AocResult<String> {
  Ok("Done!".to_string())
}

pub fn run(input: &[Vec<u8>]) -> AocResult<(String, String)> {
  Ok((part1(input)?, part2(input)?))
}
