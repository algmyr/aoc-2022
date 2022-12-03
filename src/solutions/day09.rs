use itertools::Itertools;

use crate::error::AocResult;

pub fn parse_input(fname: &str) -> AocResult<Vec<Vec<u8>>> {
  let b = std::fs::read(fname)?;
  Ok(b.split(|c| *c == b'\n').map(|bs| bs.to_vec()).collect_vec())
}

fn part1(_input: &[Vec<u8>]) -> AocResult<i32> {
  Ok(-1)
}

fn part2(_input: &[Vec<u8>]) -> AocResult<i32> {
  Ok(-1)
}

pub fn run(input: &[Vec<u8>]) -> AocResult<(i32, i32)> {
  Ok((part1(input)?, part2(input)?))
}
