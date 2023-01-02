use bstr::ByteSlice;

use error::AocResult;
use crate::utils::read_all_nums_from_bytes;

pub fn parse_input(fname: &str) -> AocResult<Vec<Vec<i32>>> {
  let f = std::fs::read(fname)?;

  f.split_str(b"\n\n")
    .map(|group| read_all_nums_from_bytes(group))
    .collect::<AocResult<Vec<Vec<i32>>>>()
}

fn part1(input: &Vec<Vec<i32>>) -> AocResult<i32> {
  let res = input.iter().map(|x| x.into_iter().sum()).max().unwrap();
  Ok(res)
}

fn top3(data: &Vec<Vec<i32>>) -> i32 {
  let mut big = 0;
  let mut medium = 0;
  let mut small = 0;

  for x in data {
    let x = x.iter().sum::<i32>();
    if x > big {
      (small, medium, big) = (medium, big, x);
    } else if x > medium {
      (small, medium) = (medium, x);
    } else if x > small {
      small = x;
    }
  }

  small + medium + big
}

fn part2(input: &Vec<Vec<i32>>) -> AocResult<i32> {
  let res = top3(input);
  Ok(res)
}

pub fn run(input: &Vec<Vec<i32>>) -> AocResult<(i32, i32)> { Ok((part1(input)?, part2(input)?)) }
