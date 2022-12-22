use std::ops::{Add, Mul, Sub, Div};

use ahash::{HashMap, HashMapExt};
use bstr::ByteSlice;
use itertools::Itertools;

use crate::error::AocResult;

#[derive(Clone, Copy)]
pub enum Command {
  Operation(u32, u8, u32),
  Constant(i64),
}

fn to_int(b: &[u8]) -> u32 {
  (b[0] as u32) << 0 | (b[1] as u32) << 8 | (b[2] as u32) << 16 | (b[3] as u32) << 24
}

impl Command {
  fn new_op(arg1: &[u8], op: &[u8], arg2: &[u8]) -> Command {
    Command::Operation(to_int(arg1), op[0], to_int(arg2))
  }
  fn new_const(arg: &[u8]) -> Command {
    Command::Constant(crate::utils::num_from_bytes(arg))
  }
}

pub fn parse_input(fname: &str) -> AocResult<Vec<(u32, Command)>> {
  let b = std::fs::read(fname)?;
  Ok(
    b.trim()
      .split(|c| *c == b'\n')
      .map(
        |bs| match bs.split(|&c| c == b' ').collect_vec().as_slice() {
          &[name, arg1, op, arg2] => (to_int(name), Command::new_op(arg1, op, arg2)),
          &[name, arg] => (to_int(name), Command::new_const(arg)),
          _ => panic!(":<"),
        },
      )
      .collect_vec(),
  )
}

fn part1(input: &[(u32, Command)]) -> AocResult<f64> {
  let mut h = HashMap::new();
  h.extend(input.iter().cloned());

  use Command::*;
  fn f(h: &HashMap<u32, Command>, name: u32) -> f64 {
    let res = match h[&name] {
      Operation(arg1, op, arg2) => {
        let arg1 = f(h, arg1);
        let arg2 = f(h, arg2);
        match op {
          b'*' => arg1 * arg2,
          b'/' => arg1 / arg2,
          b'+' => arg1 + arg2,
          b'-' => arg1 - arg2,
          _ => panic!(":<"),
        }
      }
      Constant(arg) => arg as f64,
    };
    res
  }

  let res = f(&h, to_int(b"root"));

  Ok(res)
}

#[derive(Clone, Copy, Debug)]
struct Linear {
  x_coeff: f64,
  c_coeff: f64,
}

impl Linear {
  fn new(x_coeff: f64, c_coeff: f64) -> Linear { Linear { x_coeff, c_coeff } }
}

impl Add for Linear {
  type Output = Linear;

  fn add(self, other: Self) -> Self::Output {
    Linear::new(self.x_coeff + other.x_coeff, self.c_coeff + other.c_coeff)
  }
}

impl Sub for Linear {
  type Output = Linear;

  fn sub(self, other: Self) -> Self::Output {
    Linear::new(self.x_coeff - other.x_coeff, self.c_coeff - other.c_coeff)
  }
}

impl Mul for Linear {
  type Output = Linear;

  fn mul(self, other: Self) -> Self::Output {
    let c = self.c_coeff * other.c_coeff;
    let x = self.x_coeff * other.c_coeff + self.c_coeff * other.x_coeff;
    let sq = self.x_coeff * other.x_coeff;
    assert_eq!(sq, 0.0);
    Linear::new(x, c)
  }
}

impl Div for Linear {
  type Output = Linear;

  fn div(self, other: Self) -> Self::Output {
    assert_eq!(other.x_coeff, 0.0);
    Linear::new(self.x_coeff/other.c_coeff, self.c_coeff/other.c_coeff)
  }
}

#[derive(Clone, Copy)]
enum Command2 {
  Operation(u32, u8, u32),
  Constant(Linear),
}

fn part2(input: &[(u32, Command)]) -> AocResult<i64> {
  use Command2::*;

  let mut h = HashMap::new();
  h.extend(input.iter().cloned().map(|(name, command)| match command {
    Command::Operation(a, b, c) => (name, Operation(a, b, c)),
    Command::Constant(i) => (
      name,
      Constant(Linear { x_coeff: 0.0, c_coeff: i as f64 }),
    ),
  }));

  if let Command2::Operation(arg1, _, arg2) = h[&to_int(b"root")] {
    h.insert(to_int(b"root"), Operation(arg1, b'-', arg2));
  }

  fn f(h: &HashMap<u32, Command2>, name: u32) -> Linear {
    let res = match h[&name] {
      Operation(arg1, op, arg2) => {
        let arg1 = f(h, arg1);
        let arg2 = f(h, arg2);
        match op {
          b'*' => arg1 * arg2,
          b'/' => arg1 / arg2,
          b'+' => arg1 + arg2,
          b'-' => arg1 - arg2,
          _ => panic!(":<"),
        }
      }
      Constant(arg) => arg,
    };
    res
  }

  h.insert(to_int(b"humn"), Constant(Linear::new(1.0, 0.0)));
  let expr = f(&h, to_int(b"root"));
  // ax + c = 0
  // x = -c/a
  let res = -expr.c_coeff/expr.x_coeff;
  let res = res.round() as i64;

  Ok(res)
}

pub fn run(input: &[(u32, Command)]) -> AocResult<(f64, i64)> {
  Ok((part1(input)?, part2(input)?))
}
