use bstr::ByteSlice;
use itertools::Itertools;

use crate::error::{aoc_error, AocResult};

pub enum Command {
  Addx(i32),
  Noop,
}

pub fn parse_input(fname: &str) -> AocResult<Vec<Command>> {
  let bs = std::fs::read(fname)?;
  bs.trim()
    .split(|c| *c == b'\n')
    .map(|b| match &b[..4] {
      b"addx" => {
        let arg = unsafe { std::str::from_utf8_unchecked(&b[5..]) }.parse()?;
        Ok(Command::Addx(arg))
      }
      b"noop" => Ok(Command::Noop),
      s => aoc_error(&format!("Invalid command \"{}\"", s.to_str_lossy())),
    })
    .collect()
}

fn solve(input: &[Command]) -> AocResult<(i32, String)> {
  use Command::*;

  let mut res = 0;

  let mut cycle = 0;
  let mut x = 1;
  let mut display = vec![];

  let mut draw = |pos, x| {
    display.push(pos - 1 <= x && x <= pos + 1);
  };

  for cmd in input {
    let prev_cycle = cycle;
    let prev_x = x;

    match cmd {
      Noop => {
        draw(cycle % 40, x);
        cycle += 1;
      }
      Addx(arg) => {
        draw(cycle % 40, x);
        cycle += 1;
        draw(cycle % 40, x);
        cycle += 1;
        x += arg;
      }
    }

    if (20 + prev_cycle) % 40 > (20 + cycle) % 40 {
      let aligned = cycle - cycle % 20;
      res += aligned * prev_x;
    }
  }

  let display = display
    .chunks_exact(40)
    .map(|row| {
      row
        .iter()
        .map(|b| if *b { '#' } else { '.' })
        .collect::<String>()
    })
    .join("\n");

  Ok((res, format!("\n{display}")))
}

pub fn run(input: &[Command]) -> AocResult<(i32, String)> { solve(input) }
