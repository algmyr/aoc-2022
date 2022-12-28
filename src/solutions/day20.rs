use itertools::Itertools;

use crate::error::AocResult;

pub fn parse_input(fname: &str) -> AocResult<Vec<i64>> {
  crate::utils::read_all_signed_nums(fname)
}

struct Block {
  data: Vec<usize>,
  contains: Vec<bool>,
}

impl Block {
  fn new(chunk: &[usize], numel: usize) -> Self {
    let data = chunk.to_vec();
    let mut contains = vec![false; numel];
    for x in chunk {
      contains[*x] = true;
    }
    Self { data, contains }
  }
}

struct Blocks {
  blocks: Vec<Block>,
}

impl Blocks {
  fn new(n: usize) -> Self {
    let mut blocks = vec![];
    for chunk in (0..n).chunks(100).into_iter() {
      let block = Block::new(&chunk.collect_vec(), n);
      blocks.push(block);
    }
    Self { blocks }
  }

  fn find(&self, element: usize) -> usize {
    let mut base = 0;
    for block in &self.blocks {
      if block.contains[element] {
        let ix = block.data.iter().position(|&x| x == element).expect(":<");
        return base + ix;
      }
      base += block.data.len();
    }
    unreachable!("Element not found.");
  }

  fn find_and_erase(&mut self, element: usize) -> usize {
    let mut base = 0;
    for block in &mut self.blocks {
      if block.contains[element] {
        let ix = block.data.iter().position(|&x| x == element).expect(":<");
        block.data.remove(ix);
        block.contains[element] = false;
        return base + ix;
      }
      base += block.data.len();
    }
    unreachable!("Element not found.");
  }

  fn get(&self, mut index: usize) -> usize {
    for block in &self.blocks {
      if index <= block.data.len() {
        return block.data[index];
      }
      index -= block.data.len();
    }
    unreachable!("Element at index not found.");
  }

  fn insert(&mut self, mut index: usize, element: usize) {
    for block in &mut self.blocks {
      if index <= block.data.len() {
        block.data.insert(index, element);
        block.contains[element] = true;
        break;
      }
      index -= block.data.len();
    }
  }
}

struct C<'a> {
  blocks: Blocks,
  shifts: &'a [i64],
}
impl<'a> C<'a> {
  fn new(shifts: &'a [i64]) -> Self { C { blocks: Blocks::new(shifts.len()), shifts } }

  fn shift(&mut self, i: usize, shift: i64) {
    let ix = self.blocks.find_and_erase(i);
    let n = self.shifts.len();
    let target = (ix as i64 + shift - 1).rem_euclid(n as i64 - 1) + 1;
    self.blocks.insert(target as usize, i);
  }

  fn shuffle(&mut self) {
    for i in 0..self.shifts.len() {
      self.shift(i, self.shifts[i]);
    }
  }

  fn find_zero(&self) -> usize {
    let ix = self.shifts.iter().position(|&x| x == 0).expect(":<");
    self.blocks.find(ix)
  }

  fn get_cyclic(&self, ix: usize) -> i64 {
    self.shifts[self.blocks.get(ix % self.shifts.len())]
  }
}

fn shuffle(input: &[i64], n_shuffles: usize) -> AocResult<i64> {
  let mut c = C::new(input);
  for _ in 0..n_shuffles {
    c.shuffle();
  }

  let z = c.find_zero();
  let res = c.get_cyclic(z + 1000) + c.get_cyclic(z + 2000) + c.get_cyclic(z + 3000);

  Ok(res)
}

fn part1(input: &[i64]) -> AocResult<i64> { shuffle(input, 1) }

fn part2(input: &[i64]) -> AocResult<i64> {
  let input = input.iter().map(|&v| v * 811589153).collect_vec();
  shuffle(&input, 10)
}

pub fn run(input: &[i64]) -> AocResult<(i64, i64)> { Ok((part1(input)?, part2(input)?)) }
