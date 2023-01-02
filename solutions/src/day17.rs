use std::{iter::Cycle, slice::Iter};

use bstr::ByteSlice;
use itertools::iproduct;

use error::AocResult;

pub fn parse_input(fname: &str) -> AocResult<Vec<u8>> {
  let b = std::fs::read(fname)?;
  Ok(b.trim().to_vec())
}

#[derive(Clone, Copy)]
struct Rock {
  x: isize,
  y: isize,
  pattern: u16,
  w: isize,
  h: isize,
}

impl Rock {
  fn at(&self, x: isize, y: isize) -> bool {
    let x = x - self.x;
    let y = y - self.y;
    if (0..4).contains(&x) && (0..4).contains(&y) {
      (self.pattern >> (4*y + (3 - x)) & 1) > 0
    } else { false }
  }

  fn rel_at(&self, x: isize, y: isize) -> bool {
    if (0..4).contains(&x) && (0..4).contains(&y) {
      (self.pattern >> (4*y + (3 - x)) & 1) > 0
    } else { false }
  }
}

macro_rules! rock {
  ($line0:tt $line1:tt $line2:tt $line3:tt, $w: tt, $h: tt) => {
    Rock { x: 0, y: 0, pattern: $line0 << 12 | $line1 << 8 | $line2 << 4 | $line3, w: $w, h: $h}
  };
}

struct Board {
  data: Vec<u16>,
  top: isize,
}

impl Board {
  fn new() -> Board {
    Board { data: vec![0; 5000], top: 0, }
  }
  fn at(&self, x: isize, y: isize) -> bool {
    if y < 0 {
      return true;
    }
    self.data[y as usize] >> x & 1 > 0
  }
}

#[allow(unused)]
fn draw(board: &Board, rock: &Rock) {
  //for i in (-1..8).rev() {
  for i in (board.top-6..=board.top+6).rev() {
    let mut s = String::new();
    for j in 0..7 {
      let b = board.at(j, i);
      let r = rock.at(j, i);
      if b && r {
        s.push('!');
      } else if b {
        s.push('#');
      } else if r {
        s.push('@');
      } else {
        s.push('.');
      }
    }
    println!("{}", s);
  }
}

fn simulate1(board: &mut Board, mut rock: Rock, jets: &mut Cycle<Iter<u8>>) -> i32 {
  rock.x = 2;
  rock.y = board.top + 3;

  let mut n_jets = 0;

  loop {
    let jet = jets.next().unwrap();
    n_jets += 1;

    let old = rock.x;
    if *jet == b'<' {
      rock.x = 0.max(rock.x - 1);
    } else if *jet == b'>' {
      rock.x = (7-rock.w).min(rock.x + 1);
    }
    let intersects = iproduct!(0..4, 0..4).map(|(x, y)|{
      board.at(rock.x + x, rock.y + y) && rock.rel_at(x, y)
    }).any(|x| x);
    if intersects {
      rock.x = old;
    }

    rock.y -= 1;

    let intersects = iproduct!(0..4, 0..4).map(|(x, y)|{
      board.at(rock.x + x, rock.y + y) && rock.rel_at(x, y)
    }).any(|x| x);


    if intersects {
      rock.y += 1;

      for y in 0..4 {
        for x in 0..4 {
          let px = rock.x + x;
          let py = rock.y + y;
          board.data[py as usize] |= ((rock.rel_at(x, y) as isize) << px) as u16;
        }
      }

      board.top = board.top.max(rock.y + rock.h);

      break;
    }
  }

  n_jets
}

fn part1(input: &[u8]) -> AocResult<i32> {
  let rocks = [
    rock!(0b0000
          0b0000
          0b0000
          0b1111, 4, 1),
    rock!(0b0000
          0b0100
          0b1110
          0b0100, 3, 3),
    rock!(0b0000
          0b0010
          0b0010
          0b1110, 3, 3),
    rock!(0b1000
          0b1000
          0b1000
          0b1000, 1, 4),
    rock!(0b0000
          0b0000
          0b1100
          0b1100, 2, 2),
  ];

  let rocks = rocks.into_iter().cycle();
  let mut jets = input.into_iter().cycle();

  let mut board = Board::new();

  for rock in rocks.take(2022) {
    simulate1(&mut board, rock, &mut jets);
  }

  Ok(board.top as i32)
}

fn part2(input: &[u8]) -> AocResult<i64> {
  let rocks = [
    rock!(0b0000
          0b0000
          0b0000
          0b1111, 4, 1),
    rock!(0b0000
          0b0100
          0b1110
          0b0100, 3, 3),
    rock!(0b0000
          0b0010
          0b0010
          0b1110, 3, 3),
    rock!(0b1000
          0b1000
          0b1000
          0b1000, 1, 4),
    rock!(0b0000
          0b0000
          0b1100
          0b1100, 2, 2),
  ];

  let rocks = rocks.into_iter().cycle();
  let mut jets = input.into_iter().cycle();

  let mut board = Board::new();

  // TODO(algmyr): detect cycle programmatically
  let cycle_len = 1715;
  let top_delta = 2613;

  let lim = 1000000000000;
  let to_sim = lim%cycle_len;

  for rock in rocks.take(to_sim) {
    simulate1(&mut board, rock, &mut jets);
  }

  let res = board.top as usize + (lim - to_sim)/cycle_len*top_delta;

  Ok(res as i64)
}

pub fn run(input: &[u8]) -> AocResult<(i32, i64)> {
  Ok((part1(input)?, part2(input)?))
}
