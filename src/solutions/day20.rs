use itertools::Itertools;

use crate::error::AocResult;

pub fn parse_input(fname: &str) -> AocResult<Vec<i64>> {
  crate::utils::read_all_signed_nums(fname)
}

struct C<'a> {
  indices: Vec<usize>,
  shifts: &'a [i64],
}

impl<'a> C<'a> {
  fn new(shifts: &'a [i64]) -> Self {
    C { indices: (0..shifts.len()).collect_vec(), shifts }
  }

  fn shift(&mut self, i: usize, shift: i64) {
    let n = self.indices.len();
    let ix = self.indices.iter().position(|&x| x == i).expect(":<");
    self.indices.remove(ix);
    let target = (ix as i64 + shift - 1).rem_euclid(n as i64 - 1) + 1;
    self.indices.insert(target as usize, i);
  }

  fn shuffle(&mut self) {
    for i in 0..self.indices.len() {
      self.shift(i, self.shifts[i]);
    }
  }

  fn find_zero(&self) -> usize {
    let ix = self.shifts.iter().position(|&x| x == 0).expect(":<");
    self.indices.iter().position(|&i| i == ix).expect(":<")
  }

  fn get_cyclic(&self, ix: usize) -> i64 {
    self.shifts[self.indices[ix % self.indices.len()]]
  }
}

fn shuffle(input: &[i64], n_shuffles: usize) -> AocResult<i64> {
  let mut c = C::new(input);
  for _ in 0..n_shuffles {
    c.shuffle();
  }

  let z = c.find_zero();
  let res = c.get_cyclic(z + 1000) + c.get_cyclic(z + 2000) + c.get_cyclic(z + 3000);

  Ok(res)
}

fn part1(input: &[i64]) -> AocResult<i64> { shuffle(input, 1) }

fn part2(input: &[i64]) -> AocResult<i64> {
  let input = input.iter().map(|&v| v * 811589153).collect_vec();
  shuffle(&input, 10)
}

pub fn run(input: &[i64]) -> AocResult<(i64, i64)> { Ok((part1(input)?, part2(input)?)) }
