use itertools::Itertools;

use error::AocResult;

pub fn parse_input(fname: &str) -> AocResult<Vec<(i8, i8)>> {
  let s = std::fs::read(fname)?;
  let res = s
    .chunks_exact(4)
    .map(|b| {
      let (x, y) = (b[0] - b'A', b[2] - b'X');
      (x as i8, y as i8)
    })
    .collect_vec();
  Ok(res)
}

// Lol, just look things up.
const P1: [[i8; 3]; 3] = [
  [1 + 3, 2 + 6, 3 + 0],
  [1 + 0, 2 + 3, 3 + 6],
  [1 + 6, 2 + 0, 3 + 3],
];


const P2: [[i8; 3]; 3] = [
  [3 + 0, 1 + 3, 2 + 6],
  [1 + 0, 2 + 3, 3 + 6],
  [2 + 0, 3 + 3, 1 + 6],
];

fn part1(input: &Vec<(i8, i8)>) -> AocResult<i64> {
  Ok(
    input
      .iter()
      .map(|&(other, me)| unsafe {
        *P1.get_unchecked(other as usize).get_unchecked(me as usize) as i64
      })
      .sum(),
  )
}

fn part2(input: &Vec<(i8, i8)>) -> AocResult<i64> {
  Ok(
    input
      .iter()
      .map(|&(other, me)| unsafe {
        *P2.get_unchecked(other as usize).get_unchecked(me as usize) as i64
      })
      .sum(),
  )
}

// Nice mathy way.
//
///// Assume me is 0 rock, 1 paper, 2 scissors
///// and outcome is 0 draw, 1 win, 2 loss
//fn score(me: i8, outcome: i8) -> i64 { ((me + 1) + (outcome + 1) % 3 * 3) as i64 }
//
//fn part1(input: &Vec<(i8, i8)>) -> AocResult<i64> {
//  Ok(
//    input
//      .iter()
//      .map(|&(other, me)| {
//        let outcome = (me - other).rem_euclid(3);
//        score(me, outcome)
//      })
//      .sum(),
//  )
//}
//
//fn part2(input: &Vec<(i8, i8)>) -> AocResult<i64> {
//  Ok(
//    input
//      .iter()
//      .map(|&(other, given_outcome)| {
//        let outcome = (given_outcome - 1).rem_euclid(3);
//        let me = (other + outcome).rem_euclid(3);
//        score(me, outcome)
//      })
//      .sum(),
//  )
//}

pub fn run(input: &Vec<(i8, i8)>) -> AocResult<(i64, i64)> {
  Ok((part1(input)?, part2(input)?))
}
