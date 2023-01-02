use bstr::ByteSlice;
use itertools::Itertools;

use error::AocResult;
use crate::utils::read_all_nums_from_bytes;

type InputType = ([Vec<u8>; 9], Vec<(i8, i8, i8)>);

pub fn parse_input(fname: &str) -> AocResult<InputType> {
  let b = std::fs::read(fname)?;
  let (crane, input) = b.split_once_str(b"\n\n").unwrap();

  let mut piles: [Vec<_>; 9] = Default::default();
  for s in crane.lines().rev().skip(1) {
    for (i, &c) in s.into_iter().skip(1).step_by(4).enumerate() {
      if c != b' ' {
        piles[i].push(c);
      }
    }
  }

  let instructions = read_all_nums_from_bytes(input)?
    .into_iter()
    .tuples()
    .collect_vec();

  Ok((piles, instructions))
}

fn part1(input: &InputType) -> AocResult<String> {
  let (mut piles, instructions) = input.clone();
  for (count, src, dst) in instructions {
    let src = (src - 1) as usize;
    let dst = (dst - 1) as usize;

    unsafe {
      assert_ne!(src, dst);
      let src = piles.get_unchecked_mut(src) as *mut Vec<u8>;
      let dst = piles.get_unchecked_mut(dst) as *mut Vec<u8>;
      let n = (*src).len();
      let it = (*src).drain(n - count as usize..);
      (*dst).extend(it.rev());
    }
  }

  let bytes = piles
    .into_iter()
    .map(|pile| *pile.last().unwrap_or(&b'?'))
    .collect_vec();
  let s = String::from_utf8_lossy(&bytes).into_owned();
  Ok(s)
}

fn part2(input: &InputType) -> AocResult<String> {
  let (mut piles, instructions) = input.clone();
  for (count, src, dst) in instructions {
    let src = (src - 1) as usize;
    let dst = (dst - 1) as usize;

    unsafe {
      assert_ne!(src, dst);
      let src = piles.get_unchecked_mut(src) as *mut Vec<u8>;
      let dst = piles.get_unchecked_mut(dst) as *mut Vec<u8>;
      let n = (*src).len();
      let it = (*src).drain(n - count as usize..);
      (*dst).extend(it);
    }
  }

  let bytes = piles
    .into_iter()
    .map(|pile| *pile.last().unwrap_or(&b'?'))
    .collect_vec();
  let s = String::from_utf8_lossy(&bytes).into_owned();
  Ok(s)
}

pub fn run(input: &InputType) -> AocResult<(String, String)> { Ok((part1(input)?, part2(input)?)) }
