use std::hint::unreachable_unchecked;

use bstr::ByteSlice;
use itertools::Itertools;

use error::AocResult;

#[derive(Copy, Clone)]
pub struct Blizzard {
  start: i32,
  step: i32,
}

#[derive(Clone)]
pub struct Board {
  width: i32,
  height: i32,
  cols: Vec<Vec<Blizzard>>,
  rows: Vec<Vec<Blizzard>>,
  cache: Vec<bool>,
}

impl Board {
  pub fn new(
    width: i32,
    height: i32,
    cols: Vec<Vec<Blizzard>>,
    rows: Vec<Vec<Blizzard>>,
  ) -> Self {
    Self { width, height, cols, rows, cache: vec![false; (width*height) as usize] }
  }

  fn update_occupied(&mut self, t: i32) {
    self.cache.fill(false);
    fn f(b: &Blizzard, t: i32, modulo: i32) -> i32 {
      let x = b.start + b.step * t - 1;
      x.rem_euclid(modulo) + 1
    }
    for (x, bs) in self.cols.iter().enumerate() {
      for b in bs {
        let y = f(b, t, self.height - 2);
        let ix = y*self.width + x as i32;
        self.cache[ix as usize] = true;
      }
    }
    for (y, bs) in self.rows.iter().enumerate() {
      for b in bs {
        let x = f(b, t, self.width - 2);
        let ix = y as i32*self.width + x;
        self.cache[ix as usize] = true;
      }
    }
  }

  fn occupied(&self, x: i32, y: i32) -> bool {
    let ix = y as i32*self.width + x;
    self.cache[ix as usize]
  }
}

pub fn parse_input(fname: &str) -> AocResult<Board> {
  let b = std::fs::read(fname)?;

  let lines = b.trim().split(|c| *c == b'\n').collect_vec();
  let height = lines.len();
  let width = lines[0].len();
  let mut cols = vec![vec![]; width];
  let mut rows = vec![vec![]; height];

  for (y, row) in lines.into_iter().enumerate() {
    for (x, c) in row.iter().enumerate() {
      match *c {
        b'>' => rows[y].push(Blizzard { start: x as i32, step: 1 }),
        b'<' => rows[y].push(Blizzard { start: x as i32, step: -1 }),
        b'v' => cols[x].push(Blizzard { start: y as i32, step: 1 }),
        b'^' => cols[x].push(Blizzard { start: y as i32, step: -1 }),
        _ => (),
      }
    }
  }

  Ok(Board::new(width as i32, height as i32, cols, rows))
}

fn shortest(start_t: i32, start: (i32, i32), target: (i32, i32), board: &Board) -> AocResult<i32> {
  let mut board = board.clone();

  let ok =
    move |(x, y)| (1..board.width - 1).contains(&x) && (1..board.height - 1).contains(&y);

  let mut frontier = vec![start];

  for t in start_t.. {
    board.update_occupied(t+1);
    let mut new_frontier = vec![start];

    for (x, y) in frontier {
      for neigh in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
        if neigh == target {
          return Ok(t + 1);
        }
        if ok(neigh) && !board.occupied(neigh.0, neigh.1) {
          new_frontier.push(neigh)
        }
      }
      if !board.occupied(x, y) {
        new_frontier.push((x, y))
      }
    }
    new_frontier.sort();
    new_frontier.dedup();
    frontier = new_frontier;
  }

  unsafe { unreachable_unchecked() }
}

fn solve(board: &Board) -> AocResult<(i32, i32)> {
  let start = (1, 0);
  let target = (board.width - 2, board.height - 1);

  let t1 = shortest(0, start, target, board)?;
  let t2 = shortest(t1, target, start, board)?;
  let t3 = shortest(t2, start, target, board)?;
  Ok((t1, t3))
}

pub fn run(input: &Board) -> AocResult<(i32, i32)> { solve(input) }
