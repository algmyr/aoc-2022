use itertools::Itertools;

use crate::error::AocResult;

pub fn parse_input(fname: &str) -> AocResult<Vec<Vec<i32>>> {
  let f = std::fs::read_to_string(fname)?;

  let vec = f
    .split("\n\n")
    .map(|group| {
      group
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect_vec()
    })
    .collect_vec();

  Ok(vec)
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

  small+medium+big
}

fn part2(input: &Vec<Vec<i32>>) -> AocResult<i32> {
  let res = top3(input);
  Ok(res)
}

pub fn run(input: &Vec<Vec<i32>>) -> AocResult<(i32, i32)> {
  Ok((part1(input)?, part2(input)?))
}
