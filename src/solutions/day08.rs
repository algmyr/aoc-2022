use itertools::Itertools;

use crate::error::AocResult;

pub fn parse_input(fname: &str) -> AocResult<Vec<Vec<u8>>> {
  let b = std::fs::read(fname)?;
  let mut res = b.split(|c| *c == b'\n').map(|bs| bs.to_vec()).collect_vec();
  assert!(res.pop().unwrap().is_empty());
  Ok(res)
}

fn solve(input: &[Vec<u8>]) -> AocResult<(i32, i32)> {
  let n = input.len();
  let m = input[0].len();
  let mut visible = vec![vec![false; n]; n];
  let mut scenic = vec![vec![1; n]; n];

  fn f(
    it_i: impl Iterator<Item = usize>,
    it_j: impl Iterator<Item = usize> + Clone,
    do_swap: bool,
    visible: &mut [Vec<bool>],
    scenic: &mut [Vec<i32>],
    input: &[Vec<u8>],
  ) {
    let inf = 255;
    let start = it_j.clone().next().unwrap() as i32;
    for i in it_i {
      let mut stack = vec![(start, inf)];
      let mut mx = -1;
      for j in it_j.clone() {
        let (ix_i, ix_j) = if do_swap { (j, i) } else { (i, j) };

        let h = input[ix_i][ix_j];
        if mx < h as i32 {
          mx = h as i32;
          visible[i][j] = true;
        }

        while let Some(&(_ix, ph)) = stack.last() {
          if h > ph {
            stack.pop();
          } else {
            break;
          }
        }
        if let Some(&(ix, _h)) = stack.last() {
          scenic[ix_i][ix_j] *= (j as i32 - ix).abs();
        }
        if stack.last().unwrap().1 == h {
          stack.pop();
        }
        stack.push((j as i32, h));
      }
    }
  }

  f(0..n, 0..m, false, &mut visible, &mut scenic, input);
  f(0..n, (0..m).rev(), false, &mut visible, &mut scenic, input);
  f(0..m, 0..n, true, &mut visible, &mut scenic, input);
  f(0..m, (0..n).rev(), true, &mut visible, &mut scenic, input);

  let res = visible
    .into_iter()
    .map(|v| {
      v.into_iter()
        .map(|vis| if vis { 1 } else { 0 })
        .sum::<i32>()
    })
    .sum();
  let res2 = scenic
    .into_iter()
    .map(|v| {
      v.into_iter().max().unwrap()
    })
    .max().unwrap();

  Ok((res, res2))
}

pub fn run(input: &[Vec<u8>]) -> AocResult<(i32, i32)> { solve(input) }
