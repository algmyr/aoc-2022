use itertools::Itertools;

use crate::error::AocResult;

pub fn parse_input(fname: &str) -> AocResult<Vec<Vec<(i32, i32)>>> {
  let b = std::fs::read(fname)?;
  let asd = b
    .split(|c| *c == b'\n')
    .map(|bs| {
      let ints = crate::utils::read_all_nums_from_bytes(bs)?;
      Ok(ints.into_iter().tuples().collect_vec())
    })
    .collect();
  asd
}

const HEIGHT: usize = 200;

fn draw_grid(grid: &mut [[bool; 1000]; HEIGHT], input: &[Vec<(i32, i32)>]) -> usize {
  let mut max_y = 0;
  for shape in input {
    for &[(x1, y1), (x2, y2)] in shape.array_windows() {
      max_y = max_y.max(y1).max(y2);
      if x1 == x2 {
        // Lol, I guess this works
        for y in y1..=y2 {
          grid[y as usize][x1 as usize] = true;
        }
        for y in y2..=y1 {
          grid[y as usize][x1 as usize] = true;
        }
      } else if y1 == y2 {
        for x in x1..=x2 {
          grid[y1 as usize][x as usize] = true;
        }
        for x in x2..=x1 {
          grid[y1 as usize][x as usize] = true;
        }
      }
    }
  }
  let max_y = max_y as usize + 2;

  max_y
}

fn part1(input: &[Vec<(i32, i32)>]) -> AocResult<i32> {
  let mut grid = [[false; 1000]; HEIGHT];
  let max_y = draw_grid(&mut grid, input);

  let mut res = 0;
  let mut stack = vec![(500, 0)];
  loop {
    let &(x, y) = stack.last().unwrap();
    if y >= max_y - 1 {
      break;
    }

    if !grid[y + 1][x] {
      stack.push((x, y + 1));
    } else if !grid[y + 1][x - 1] {
      stack.push((x - 1, y + 1));
    } else if !grid[y + 1][x + 1] {
      stack.push((x + 1, y + 1));
    } else {
      res += 1;
      grid[y][x] = true;
      stack.pop();
    }
  }

  Ok(res)
}

#[allow(unused)]
fn part2(input: &[Vec<(i32, i32)>]) -> AocResult<i32> {
  let mut grid = [[false; 1000]; HEIGHT];
  let max_y = draw_grid(&mut grid, input);

  for x in 0..1000 {
    grid[max_y][x] = true;
  }

  let mut res = 0;
  let mut stack = vec![(500, 0)];
  stack.reserve(100);
  loop {
    let Some(&(x, y)) = stack.last() else { break; };

    if !grid[y + 1][x] {
      stack.push((x, y + 1));
    } else if !grid[y + 1][x - 1] {
      stack.push((x - 1, y + 1));
    } else if !grid[y + 1][x + 1] {
      stack.push((x + 1, y + 1));
    } else {
      res += 1;
      grid[y][x] = true;
      stack.pop();
    }
  }

  Ok(res)
}

fn part2_alt(input: &[Vec<(i32, i32)>]) -> AocResult<i32> {
  let mut grid = [[false; 1000]; HEIGHT];
  let mut grains = [[false; 1000]; HEIGHT];
  let max_y = draw_grid(&mut grid, input);

  let mut min_x = 500;
  let mut max_x = 501;

  let mut res = 1;
  grains[0][500] = true;

  fn f(res: &mut i32, x: usize, y: usize, grains: &mut [[bool; 1000]; HEIGHT], grid: &[[bool; 1000]; HEIGHT]) -> bool {
    if !grid[y][x] {
      let a = grains[y-1][x-1];
      let b = grains[y-1][x];
      let c = grains[y-1][x+1];
      let to_fill = a || b || c;
      *res += to_fill as i32;
      grains[y][x] = to_fill;
      to_fill
    } else {
      false
    }
  }

  for y in 1..max_y {
    for x in min_x..max_x {
      f(&mut res, x, y, &mut grains, &grid);
    }
    if f(&mut res, min_x-1, y, &mut grains, &grid) {
      min_x -= 1;
    }
    if f(&mut res, max_x, y, &mut grains, &grid) {
      max_x += 1;
    }
  }
  Ok(res)
}

pub fn run(input: &[Vec<(i32, i32)>]) -> AocResult<(i32, i32)> {
  Ok((part1(input)?, part2_alt(input)?))
}
