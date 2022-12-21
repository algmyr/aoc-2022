use itertools::Itertools;

use crate::error::AocResult;

pub fn parse_input(fname: &str) -> AocResult<Vec<i64>> {
  crate::utils::read_all_signed_nums(fname)
}

fn shuffle(input: &[i64], n_shuffles: usize) -> AocResult<i64> {
  let n = input.len();
  let mut indices = (0..n).collect_vec();

  for _ in 0..n_shuffles {
    for i in 0..n {
      let ix = indices.iter().position(|&x| x == i).expect(":<");
      let shift = input[i];
      indices.remove(ix);
      let target = (ix as i64 + shift - 1).rem_euclid(n as i64 - 1) + 1;
      indices.insert(target as usize, i);
    }
  }

  let z_ix = input.iter().position(|&x| x == 0).expect(":<");
  let z = indices.iter().position(|&i| i == z_ix).expect(":<");
  let res = input[indices[(z + 1000) % n]]
    + input[indices[(z + 2000) % n]]
    + input[indices[(z + 3000) % n]];

  Ok(res)
}

fn part1(input: &[i64]) -> AocResult<i64> {
  shuffle(input, 1)
}

fn part2(input: &[i64]) -> AocResult<i64> {
  let input = input.iter().map(|&v| v*811589153).collect_vec();
  shuffle(&input, 10)
}

pub fn run(input: &[i64]) -> AocResult<(i64, i64)> { Ok((part1(input)?, part2(input)?)) }
