use std::fmt::Display;

use crate::error::AocResult;

#[allow(dead_code)]
fn parse_input(fname: &str) -> AocResult<()> {
  let _s = std::fs::read_to_string(fname)?;
  Ok(())
}

fn part1(_fname: &str) -> AocResult<i32> {
  Ok(-1)
}

fn part2(_fname: &str) -> AocResult<i32> {
  Ok(-1)
}

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> {
  Ok((part1(fname)?, part2(fname)?))
}
