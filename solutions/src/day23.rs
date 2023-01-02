use std::ops::{Index, IndexMut};

use bstr::ByteSlice;
use itertools::Itertools;

use error::AocResult;

#[derive(Clone, Copy, Debug)]
enum Dir {
  NONE,
  N,
  S,
  W,
  E,
}

impl Dir {
  fn value(&self) -> Option<(i32, i32)> {
    match self {
      Dir::N => Some((0, -1)),
      Dir::S => Some((0, 1)),
      Dir::W => Some((-1, 0)),
      Dir::E => Some((1, 0)),
      Dir::NONE => None,
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct Elf {
  x: i32,
  y: i32,
  step: Dir,
}

impl Elf {
  fn moved(&self) -> Option<(i32, i32)> {
    self.step.value().map(|(dx, dy)| (self.x + dx, self.y + dy))
  }
}

pub fn parse_input(fname: &str) -> AocResult<(Vec<Elf>, usize, usize)> {
  let b = std::fs::read(fname)?;

  let mut res = vec![];
  let mut width = 0;
  let mut height = 0;
  for (y, row) in b.trim().split(|c| *c == b'\n').enumerate() {
    width = row.len();
    height += 1;
    for (x, c) in row.iter().enumerate() {
      if *c == b'#' {
        res.push(Elf { x: x as i32, y: y as i32, step: Dir::NONE })
      }
    }
  }

  Ok((res, width, height))
}

type IndexType = i32;
struct Space<T, const N: usize> {
  data: [[T; N]; N],
}
impl<T: Default + Copy, const N: usize> Space<T, N> {
  fn new() -> Self { Self { data: [[T::default(); N]; N] } }
}
impl<T, const N: usize> Index<[IndexType; 2]> for Space<T, N> {
  type Output = T;

  fn index(&self, index: [IndexType; 2]) -> &Self::Output {
    let [x, y] = index;
    unsafe {
      self
        .data
        .get_unchecked(x as usize)
        .get_unchecked(y as usize)
    }
  }
}
impl<T, const N: usize> IndexMut<[IndexType; 2]> for Space<T, N> {
  fn index_mut(&mut self, index: [IndexType; 2]) -> &mut Self::Output {
    let [x, y] = index;
    unsafe {
      self
        .data
        .get_unchecked_mut(x as usize)
        .get_unchecked_mut(y as usize)
    }
  }
}

const SIZE: usize = 500;

struct Simulation {
  occupied: Space<bool, SIZE>,
  count: Space<u8, SIZE>,
  dirs: [(u8, Dir); 4],
}

impl Simulation {
  fn new() -> Self {
    let dirs = [
      (0b11100000_u8, Dir::N),
      (0b00001110_u8, Dir::S),
      (0b10000011_u8, Dir::W),
      (0b00111000_u8, Dir::E),
    ];
    Self { occupied: Space::<bool, SIZE>::new(), count: Space::<u8, SIZE>::new(), dirs }
  }

  fn mark_occupied(&mut self, elves: &[Elf]) {
    for elf in elves {
      self.occupied[[elf.x, elf.y]] = true;
    }
  }

  fn propose_moves(&mut self, elves: &mut [Elf]) {
    fn neighbors(occupied: &Space<bool, SIZE>, x: i32, y: i32) -> u8 {
      // 012
      // 7 3
      // 654
      let get = |x, y| occupied[[x, y]];
      (get(x - 1, y - 1) as u8) << 7
        | (get(x + 0, y - 1) as u8) << 6
        | (get(x + 1, y - 1) as u8) << 5
        | (get(x + 1, y + 0) as u8) << 4
        | (get(x + 1, y + 1) as u8) << 3
        | (get(x + 0, y + 1) as u8) << 2
        | (get(x - 1, y + 1) as u8) << 1
        | (get(x - 1, y + 0) as u8) << 0
    }

    for elf in elves {
      let x = elf.x;
      let y = elf.y;

      let neigh = neighbors(&self.occupied, x, y);

      if neigh != 0 {
        for &(mask, dir) in &self.dirs {
          if neigh & mask == 0 {
            elf.step = dir;
            let (nx, ny) = elf.moved().unwrap();
            self.count[[nx, ny]] += 1;
            break;
          }
        }
      }
    }
  }

  fn perform_moves(&mut self, elves: &mut [Elf]) -> i32 {
    let mut n_moves = 0;
    // Move.
    for elf in elves {
      if let Some((nx, ny)) = elf.moved() {
        self.occupied[[elf.x, elf.y]] = false;
        if self.count[[nx, ny]] == 1 {
          elf.x = nx;
          elf.y = ny;
          n_moves += 1;
        }
        self.count[[nx, ny]] = 0;
        elf.step = Dir::NONE;
      }
    }
    n_moves
  }

  fn clear_unmoved(&mut self, elves: &mut [Elf]) {
    for elf in elves {
      self.occupied[[elf.x, elf.y]] = false;
      if let Some((nx, ny)) = elf.moved() {
        self.count[[nx, ny]] = 0;
      }
    }
  }

  fn simulate(&mut self, elves: &mut [Elf]) -> i32 {
    self.mark_occupied(elves);
    self.propose_moves(elves);
    let n_moves = self.perform_moves(elves);
    self.clear_unmoved(elves);
    self.dirs.rotate_left(1);
    n_moves
  }
}

fn part1(input: &[Elf], width: usize, height: usize) -> AocResult<i32> {
  let mut sim = Simulation::new();

  let mut elves = input
    .iter()
    .map(|elf| Elf {
      x: elf.x + (SIZE / 2 - width / 2) as i32,
      y: elf.y + (SIZE / 2 - height / 2) as i32,
      step: elf.step,
    })
    .collect_vec();

  for _ in 0..10 {
    sim.simulate(&mut elves);
  }

  let mut min_x = SIZE as i32;
  let mut min_y = SIZE as i32;
  let mut max_x = 0;
  let mut max_y = 0;
  for elf in &elves {
    min_x = min_x.min(elf.x);
    min_y = min_y.min(elf.y);
    max_x = max_x.max(elf.x);
    max_y = max_y.max(elf.y);
  }
  let res = (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i32;

  Ok(res)
}

fn part2(input: &[Elf], width: usize, height: usize) -> AocResult<i32> {
  let mut sim = Simulation::new();

  let mut elves = input
    .iter()
    .map(|elf| Elf {
      x: elf.x + (SIZE / 2 - width / 2) as i32,
      y: elf.y + (SIZE / 2 - height / 2) as i32,
      step: elf.step,
    })
    .collect_vec();

  let mut sim_it = 1;
  while sim.simulate(&mut elves) != 0 {
    sim_it += 1;
  }
  Ok(sim_it)
}

pub fn run(input: &(Vec<Elf>, usize, usize)) -> AocResult<(i32, i32)> {
  let (elves, width, height) = input;
  Ok((
    part1(&elves, *width, *height)?,
    part2(&elves, *width, *height)?,
  ))
}
