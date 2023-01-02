use std::hint::unreachable_unchecked;

use bstr::ByteSlice;
use itertools::Itertools;

use error::AocResult;

type Input = (u8, i32);

pub fn parse_input(fname: &str) -> AocResult<Vec<Input>> {
  let bs = std::fs::read(fname)?;
  Ok(
    bs.trim()
      .split(|c| *c == b'\n')
      .map(|b| {
        let dir = b[0];
        let count = if b[2] == b'-' {
          -crate::utils::num_from_bytes::<i32>(&b[3..])
        } else {
          crate::utils::num_from_bytes(&b[2..])
        };
        (dir, count)
      })
      .collect_vec(),
  )
}

type PointInt = i32;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Pt {
  x: PointInt,
  y: PointInt,
}

#[derive(Debug)]
struct Snek<const N: usize> {
  head: Pt,
  tail: [Pt; N],
}

impl<const N: usize> Snek<N> {
  fn new() -> Self { Snek { head: Pt{x:0, y:0}, tail: [Pt{x:0, y:0}; N] } }
  fn up(&mut self) { self.head.y -= 1; }
  fn down(&mut self) { self.head.y += 1; }
  fn left(&mut self) { self.head.x -= 1; }
  fn right(&mut self) { self.head.x += 1; }

  fn end(&self) -> Pt { *self.tail.last().unwrap() }

  fn follow(&mut self) {
    let mut last = self.head;
    for cur in &mut self.tail {
      let dx = last.x - cur.x;
      let dy = last.y - cur.y;
      let distx = dx.abs();
      let disty = dy.abs();
      if distx.max(disty) >= 2 {
        cur.x += dx.signum();
        cur.y += dy.signum();
      } else {
        break;
      }
      last = *cur;
    }
  }
}

struct Board {
  grid: Vec<bool>,
  n: usize,
  uniq: i32,
}

impl Board {
  fn new(n: usize) -> Self {
    Board { grid: vec![false; (2*n+1)*(2*n+1)], n, uniq: 0 }
  }
  fn insert(&mut self, p: Pt) {
    let n = self.n as i32;
    let ix = (p.x as i32+n) + (p.y as i32+n)*(2*n + 1);
    if !self.grid[ix as usize] {
      self.uniq += 1;
      self.grid[ix as usize] = true;
    }
  }
  fn len(&self) -> i32 { self.uniq }
}

fn solve<const N: usize>(input: &[Input]) -> AocResult<i32> {
  let mut points = Board::new(300);
  let mut snek = Snek::<N>::new();
  for &(dir, count) in input {
    let dir = match dir {
      b'R' => Snek::right,
      b'D' => Snek::down,
      b'L' => Snek::left,
      b'U' => Snek::up,
      _ => unsafe { unreachable_unchecked() },
    };

    for _ in 0..count {
      dir(&mut snek);
      snek.follow();
      points.insert(snek.end());
    }
  }
  Ok(points.len() as i32)
}

fn part1(input: &[Input]) -> AocResult<i32> {
  solve::<1>(input)
}

fn part2(input: &[Input]) -> AocResult<i32> {
  solve::<9>(input)
}

pub fn run(input: &[Input]) -> AocResult<(i32, i32)> { Ok((part1(input)?, part2(input)?)) }
