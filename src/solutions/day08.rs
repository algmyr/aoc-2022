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

  let mut visible = vec![vec![false; m]; n];
  let mut scenic = vec![vec![1; m]; n];

  macro_rules! sweep {
    ($it_i: expr, $it_j: expr, $i: ident, $j: ident, $ix_i: ident, $ix_j: ident) => {
      let start = $it_j.clone().next().unwrap() as i32;
      let inf = 255;
      for $i in $it_i {
        let mut mx = -1; // Part 1
        let mut stack = vec![(start, inf)]; // Part 2
        for $j in $it_j.clone() {
          let h = input[$ix_i][$ix_j];

          // Part 1: Compare with max so far.
          if mx < h as i32 {
            mx = h as i32;
            visible[$i][$j] = true;
          }

          // Part 2: Use monotonic stack to find the closest tree with
          //         height <= our tree. O(1) amortized.
          while let Some(&(_ix, ph)) = stack.last() {
            if h > ph {
              stack.pop();
            } else {
              break;
            }
          }
          if let Some(&(ix, ph)) = stack.last() {
            scenic[$ix_i][$ix_j] *= ($j as i32 - ix).abs();
            if ph == h {
              stack.pop();
            }
          }
          stack.push(($j as i32, h));
        }
      }
    };

    (i = $seq1: expr, j = $seq2: expr) => {
      sweep!($seq1, $seq2, i, j, i, j);
    };
    (j = $seq1: expr, i = $seq2: expr) => {
      sweep!($seq1, $seq2, i, j, j, i);
    };
  }

  sweep!(i = 0..n, j = 0..m);
  sweep!(i = 0..n, j = (0..m).rev());
  sweep!(j = 0..m, i = 0..n);
  sweep!(j = 0..m, i = (0..n).rev());

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
    .map(|v| v.into_iter().max().unwrap())
    .max()
    .unwrap();

  Ok((res, res2))
}

pub fn run(input: &[Vec<u8>]) -> AocResult<(i32, i32)> { solve(input) }
