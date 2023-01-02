use std::ops::{Index, IndexMut};

use itertools::Itertools;

use error::AocResult;
use crate::utils::read_all_nums_from_bytes;

pub fn parse_input(fname: &str) -> AocResult<Vec<(i8, i8, i8)>> {
  let b = std::fs::read(fname)?;
  let res = read_all_nums_from_bytes(&b)?
    .into_iter()
    .tuples()
    .collect_vec();
  Ok(res)
}

const N: usize = 30;
struct Space {
  seen: [[[bool; N]; N]; N],
}
impl Space {
  fn new() -> Space { Space { seen: [[[false; N]; N]; N] } }
}
impl Index<[i8; 3]> for Space {
  type Output = bool;

  fn index(&self, index: [i8; 3]) -> &Self::Output {
    let [x, y, z] = index;
    unsafe {
      self
        .seen
        .get_unchecked(x as usize)
        .get_unchecked(y as usize)
        .get_unchecked(z as usize)
    }
  }
}
impl IndexMut<[i8; 3]> for Space {
  fn index_mut(&mut self, index: [i8; 3]) -> &mut Self::Output {
    let [x, y, z] = index;
    unsafe {
      self
        .seen
        .get_unchecked_mut(x as usize)
        .get_unchecked_mut(y as usize)
        .get_unchecked_mut(z as usize)
    }
  }
}

fn solve(input: &[(i8, i8, i8)]) -> AocResult<(usize, usize)> {
  let mut seen = Space::new();

  let mut res = 0;
  for &(x, y, z) in input {
    let x = x + 1;
    let y = y + 1;
    let z = z + 1;

    res += 6;
    for (dx, dy, dz) in [
      (1, 0, 0),
      (0, 1, 0),
      (0, 0, 1),
      (-1, 0, 0),
      (0, -1, 0),
      (0, 0, -1),
    ]{
      let p = [x+dx, y+dy, z+dz];
      seen[[x, y, z]] = true;
      if seen[p] {
        res -= 2;
      }
    }
  }

  let mut res2 = 0;
  let mut stack = vec![[0, 0 ,0]];
  let mut visited = Space::new();
  let r = 0..(N as i8);
  while let Some(p) = stack.pop() {
    let [x, y, z] = p;
    if visited[p] {
      continue;
    }
    visited[p] = true;

    for [dx, dy, dz] in [
      [1, 0, 0],
      [0, 1, 0],
      [0, 0, 1],
      [-1, 0, 0],
      [0, -1, 0],
      [0, 0, -1],
    ]{
      let q = [x+dx, y+dy, z+dz];
      if r.contains(&q[0]) && r.contains(&q[1]) && r.contains(&q[2]) {
        if !seen[q] {
          stack.push(q);
        } else {
          res2 += 1;
        }
      }
    }
  }

  Ok((res, res2))
}

pub fn run(input: &[(i8, i8, i8)]) -> AocResult<(usize, usize)> { solve(input) }
