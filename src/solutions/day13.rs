use std::cmp::Ordering;

use bstr::ByteSlice;
use itertools::Itertools;

use crate::error::AocResult;

#[derive(Debug, Clone, Eq)]
pub enum Thing {
  Int(i32),
  List(Vec<Thing>),
}

impl Ord for Thing {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (Thing::Int(x), Thing::Int(y)) => return x.cmp(y),
      (Thing::Int(i), l @ Thing::List(_)) => Thing::List(vec![Thing::Int(*i)]).cmp(l),
      (l @ Thing::List(_), Thing::Int(i)) => l.cmp(&Thing::List(vec![Thing::Int(*i)])),
      (Thing::List(a), Thing::List(b)) => {
        let mut a_it = a.into_iter();
        let mut b_it = b.into_iter();
        loop {
          let x = a_it.next();
          let y = b_it.next();
          match (x, y) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(s), Some(t)) => {
              let res = s.cmp(t);
              if res != Ordering::Equal {
                return res;
              }
            }
          }
        }
      }
    }
  }
}

impl PartialOrd for Thing {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl PartialEq for Thing {
  fn eq(&self, other: &Self) -> bool { self.cmp(other) == Ordering::Equal }
}

fn parse(start: usize, bs: &[u8]) -> (usize, Thing) {
  let mut i = start;
  let b = bs[i];

  if b.is_ascii_digit() {
    let mut n = 0;
    while bs[i].is_ascii_digit() {
      n = n * 10 + (bs[i] - b'0') as i32;
      i += 1;
    }
    return (i, Thing::Int(n));
  }

  let mut v = vec![];
  while bs[i] != b']' {
    i += 1;
    if bs[i] == b']' {
      break;
    }
    let (new_i, thing) = parse(i, bs);
    v.push(thing);
    i = new_i;
  }
  (i + 1, Thing::List(v))
}

pub fn parse_input(fname: &str) -> AocResult<Vec<Thing>> {
  let b = std::fs::read(fname)?;
  let res = b
    .trim()
    .split_str(b"\n")
    .filter(|s| !s.is_empty())
    .map(|s| {
      parse(0, s).1
    })
    .collect_vec();
  Ok(res)
}

fn part1(input: &[Thing]) -> AocResult<usize> {
  let mut res = 0;
  for (i, (x, y)) in input.iter().tuples().enumerate() {
    if x < y {
      res += i + 1;
    }
  }
  Ok(res)
}

fn part2(input: &[Thing]) -> AocResult<usize> {
  let a = Thing::List(vec![Thing::Int(2)]);
  let b = Thing::List(vec![Thing::Int(6)]);

  let mut input = input.iter().collect_vec();
  input.push(&a);
  input.push(&b);

  input.sort_unstable();
  let i = input.binary_search(&&a).unwrap();
  let j = input.binary_search(&&b).unwrap();
  Ok((i + 1) * (j + 1))
}

pub fn run(input: &[Thing]) -> AocResult<(usize, usize)> {
  Ok((part1(input)?, part2(input)?))
}
