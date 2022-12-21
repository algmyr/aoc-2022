use std::collections::HashSet;

use itertools::Itertools;

use crate::{error::AocResult, utils::read_all_nums_from_bytes};

pub fn parse_input(fname: &str) -> AocResult<Vec<(i8, i8, i8)>> {
  let b = std::fs::read(fname)?;
  let res = read_all_nums_from_bytes(&b)?.into_iter().tuples().collect_vec();
  Ok(res)
}

fn solve(input: &[(i8, i8, i8)]) -> AocResult<(usize, usize)> {
  let mut h = HashSet::new();
  let mut res = 0;
  for &(x, y, z) in input {
    res += 6;
    for (dx, dy, dz) in [
      (1, 0, 0),
      (0, 1, 0),
      (0, 0, 1),
      (-1, 0, 0),
      (0, -1, 0),
      (0, 0, -1),
    ]{
      let p = (x+dx, y+dy, z+dz);
      h.insert((x, y, z));
      if h.contains(&p) {
        res -= 2;
      }
    }
  }

  const N: i8 = 30;

  let mut res2 = 0;
  let mut stack = vec![(N,N,N)];
  let mut visited = HashSet::new();
  while let Some(p) = stack.pop() {
    let (x, y, z) = p;
    if !visited.insert(p) {
      continue;
    }

    for (dx, dy, dz) in [
      (1, 0, 0),
      (0, 1, 0),
      (0, 0, 1),
      (-1, 0, 0),
      (0, -1, 0),
      (0, 0, -1),
    ]{
      let q = (x+dx, y+dy, z+dz);
      if (-N..=N).contains(&q.0) && (-N..=N).contains(&q.1) && (-N..=N).contains(&q.2) {
        if !h.contains(&q) {
          stack.push(q);
        } else {
          res2 += 1;
        }
      }
    }
  }

  Ok((res, res2))
}

pub fn run(input: &[(i8, i8, i8)]) -> AocResult<(usize, usize)> {
  solve(input)
}
