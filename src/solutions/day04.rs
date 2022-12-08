use itertools::Itertools;

use crate::error::AocResult;
use crate::utils;

pub fn parse_input(fname: &str) -> AocResult<Vec<[i8; 4]>> {
  let ints = utils::read_all_nums(fname)?;
  let res = ints.into_iter().array_chunks::<4>().collect_vec();
  Ok(res)
}

struct Range {
  l: i8,
  r: i8,
}

impl Range {
  fn new(l: i8, r: i8) -> Self { Range { l, r } }
  fn intersects(&self, other: &Range) -> bool { !(self.r < other.l || other.r < self.l) }
  fn contains(&self, other: &Range) -> bool {
    (self.l <= other.l && other.r <= self.r) || (other.l <= self.l && self.r <= other.r)
  }
}

fn solve(input: &[[i8; 4]]) -> AocResult<(i32, i32)> {
  let mut res1 = 0;
  let mut res2 = 0;
  for &[l1, r1, l2, r2] in input {
    let r1 = Range::new(l1, r1);
    let r2 = Range::new(l2, r2);
    if r1.contains(&r2) {
      res1 += 1;
    }
    if r1.intersects(&r2) {
      res2 += 1;
    }
  }
  Ok((res1, res2))
}

pub fn run(input: &[[i8; 4]]) -> AocResult<(i32, i32)> { Ok(solve(input)?) }
