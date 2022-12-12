use std::collections::VecDeque;

use bstr::ByteSlice;
use itertools::Itertools;

use crate::error::{aoc_error, AocResult};

pub fn parse_input(fname: &str) -> AocResult<Vec<Vec<u8>>> {
  let b = std::fs::read(fname)?;
  Ok(
    b.trim()
      .split(|c| *c == b'\n')
      .map(|bs| bs.to_vec())
      .collect_vec(),
  )
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
struct Pt {
  x: i32,
  y: i32,
}

impl Pt {
  fn new(x: i32, y: i32) -> Self { Pt { x, y } }
}

fn bfs(input: &[Vec<u8>], starts: Vec<Pt>, target: Pt) -> AocResult<i32> {
  let mut dq = VecDeque::new();
  dq.extend(starts.into_iter().map(|pt| (0, pt, b'a')));

  let mut visited = vec![vec![false; input[0].len()]; input.len()];

  while let Some(v) = dq.pop_front() {
    let (dist, cur, h) = v;

    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
      let p = Pt::new(cur.x + dx, cur.y + dy);
      if let Some(new_h) = input
        .get(p.y as usize)
        .and_then(|row| row.get(p.x as usize))
      {
        if h >= new_h - 1 {
          if p == target {
            return Ok(dist + 1);
          }
          if !visited[p.y as usize][p.x as usize] {
            visited[p.y as usize][p.x as usize] = true;
            dq.push_back((dist + 1, p, *new_h));
          }
        }
      }
    }
  }

  aoc_error("No answer found")
}

fn part1(input: &[Vec<u8>], start: Pt, target: Pt) -> AocResult<i32> {
  bfs(input, vec![start], target)
}

fn part2(input: &[Vec<u8>], starts: Vec<Pt>, target: Pt) -> AocResult<i32> {
  bfs(input, starts, target)
}

fn prep_input(mut input: Vec<Vec<u8>>) -> (Vec<Vec<u8>>, Pt, Vec<Pt>, Pt) {
  let mut start = Pt::new(-1, -1);
  let mut target = Pt::new(-1, -1);
  let mut a_targets = vec![];
  for (y, row) in input.iter_mut().enumerate() {
    for (x, c) in row.iter_mut().enumerate() {
      if *c == b'S' {
        start = Pt::new(x as i32, y as i32);
        *c = b'a';
      }
      if *c == b'E' {
        target = Pt::new(x as i32, y as i32);
        *c = b'z';
      }
      if *c == b'a' {
        a_targets.push(Pt::new(x as i32, y as i32));
      }
    }
  }
  (input, start, a_targets, target)
}

pub fn run(input: &[Vec<u8>]) -> AocResult<(i32, i32)> {
  let (input, start, a_starts, target) = prep_input(input.to_vec());

  Ok((
    part1(&input, start, target)?,
    part2(&input, a_starts, target)?,
  ))
}
