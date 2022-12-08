use crate::error::AocResult;

pub fn parse_input(fname: &str) -> AocResult<Vec<u8>> {
  Ok(std::fs::read(fname)?)
}

struct DupCounter {
  count: [u8; 256],
  dupes: i32,
}

impl DupCounter {
  fn new() -> Self{
    DupCounter { count: [0; 256], dupes: 0 }
  }
  fn add(&mut self, b: u8) {
    self.count[b as usize] += 1; 
    if self.count[b as usize] == 2 {
      self.dupes += 1;
    }
  }
  fn remove(&mut self, b: u8) {
    self.count[b as usize] -= 1; 
    if self.count[b as usize] == 1 {
      self.dupes -= 1;
    }
  }
}

fn solve(input: &[u8], n: usize) -> AocResult<i32> {
  let mut counter = DupCounter::new();

  for i in 0..n {
    counter.add(input[i]);
  }

  let mut res = n as i32;
  for window in input.windows(n+1) {
    counter.remove(window[0]);
    counter.add(window[n]);

    res += 1;
    if counter.dupes == 0 {
      break;
    }
  }
  Ok(res)
}

fn part1(input: &[u8]) -> AocResult<i32> {
  solve(input, 4)
}

fn part2(input: &[u8]) -> AocResult<i32> {
  solve(input, 14)
}

pub fn run(input: &[u8]) -> AocResult<(i32, i32)> {
  Ok((part1(input)?, part2(input)?))
}
