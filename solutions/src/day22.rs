use std::hint::unreachable_unchecked;
use std::ops::Range;

use bstr::ByteSlice;
use derive_more::{Add, AddAssign, Constructor, Sub, SubAssign};
use itertools::Itertools;

use error::AocResult;

pub fn parse_input(fname: &str) -> AocResult<Vec<Vec<u8>>> {
  let b = std::fs::read(fname)?;
  Ok(
    b.trim_end()
      .split(|c| *c == b'\n')
      .map(|bs| bs.to_vec())
      .collect_vec(),
  )
}

#[derive(Clone, Copy)]
enum Action {
  Forward(usize),
  Left,
  Right,
}

fn parse_instructions(instruction_bytes: Vec<u8>) -> Vec<Action> {
  let mut instructions = vec![];
  let mut steps = 0;
  for b in instruction_bytes {
    if b.is_ascii_digit() {
      steps = 10 * steps + (b - b'0') as usize;
      continue;
    }
    if steps > 0 {
      instructions.push(Action::Forward(steps));
      steps = 0;
    }
    if b == b'L' {
      instructions.push(Action::Left);
    } else if b == b'R' {
      instructions.push(Action::Right);
    } else {
      panic!("AAA");
    }
  }
  if steps > 0 {
    instructions.push(Action::Forward(steps));
  }
  instructions
}

#[derive(Clone, Copy)]
struct Heading {
  heading: i32,
}
impl Heading {
  fn rot(&self, n: i32) -> Heading {
    Heading { heading: (self.heading + n).rem_euclid(4) }
  }
  fn dir(&self) -> Point {
    match self.heading {
      0 => Point::new(1, 0),
      1 => Point::new(0, 1),
      2 => Point::new(-1, 0),
      3 => Point::new(0, -1),
      _ => panic!("where are you heading? :<"),
    }
  }
  fn value(&self) -> i32 { self.heading }
}

#[derive(Debug, Clone, Copy, Add, Sub, AddAssign, SubAssign, Constructor)]
struct Point {
  x: i32,
  y: i32,
}

#[allow(unused)]
fn draw_board(data: &[Vec<u8>], pos: Point, rad: i32, heading: Heading) {
  let mut out = vec![];
  let min_x = pos.x - rad;
  let max_x = pos.x + rad;
  let min_y = pos.y - rad;
  let max_y = pos.y + rad;
  for y in min_y..=max_y {
    let mut s = String::new();
    for x in min_x..=max_x {
      let c = *data
        .get(y as usize)
        .and_then(|row| row.get(x as usize))
        .unwrap_or(&b' ');
      if x == pos.x && y == pos.y {
        assert!(c == b'.');
        s.push(b">v<^"[heading.value() as usize].into());
      } else {
        s.push(c.into());
      }
    }
    out.push(s);
  }
  println!("{}", out.join("\n"));
}

trait Board {
  fn step(&self, pos: Point, heading: Heading) -> Option<(Point, Heading)>;
}

struct WrapBoard {
  data: Vec<Vec<u8>>,
  xlimits: Vec<Range<i32>>,
  ylimits: Vec<Range<i32>>,
}
impl WrapBoard {
  fn new(input: &[Vec<u8>]) -> WrapBoard {
    let mut xlimits = vec![];
    let mut ylimits = vec![];
    for row in input {
      let first = row.iter().position(|&b| b != b' ').unwrap();
      let last = row.len(); // Assume no trailing whitespace
      xlimits.push(first as i32..last as i32);
    }
    let max_x = xlimits.iter().map(|r| r.end).max().unwrap() as usize;
    for x in 0..max_x {
      let mut first = None;
      let mut last = None;
      for y in 0..input.len() {
        if let Some(&b) = input[y].get(x) {
          if b != b' ' {
            first = first.or(Some(y));
            last = Some(y);
          }
        }
      }
      ylimits.push(first.unwrap() as i32..last.unwrap() as i32 + 1);
    }
    WrapBoard { data: input.to_vec(), xlimits, ylimits }
  }
}
impl Board for WrapBoard {
  fn step(&self, pos: Point, heading: Heading) -> Option<(Point, Heading)> {
    fn wrap(x: i32, range: &Range<i32>) -> i32 {
      (x - range.start).rem_euclid(range.len() as i32) + range.start
    }

    let new = pos + heading.dir();
    let new = Point::new(
      wrap(new.x, &self.xlimits[pos.y as usize]),
      wrap(new.y, &self.ylimits[pos.x as usize]),
    );
    if self.data[new.y as usize][new.x as usize] == b'.' {
      Some((new, heading))
    } else {
      None
    }
  }
}

const CUBE_SIZE: i32 = 50;

struct CubeBoard {
  data: Vec<Vec<u8>>,
}
impl CubeBoard {
  fn new(input: &[Vec<u8>]) -> Self {
    let mut xlimits = vec![];
    let mut ylimits = vec![];
    for row in input {
      let first = row.iter().position(|&b| b != b' ').unwrap();
      let last = row.len(); // Assume no trailing whitespace
      xlimits.push(first as i32..last as i32);
    }
    let max_x = xlimits.iter().map(|r| r.end).max().unwrap() as usize;
    for x in 0..max_x {
      let mut first = None;
      let mut last = None;
      for y in 0..input.len() {
        if let Some(&b) = input[y].get(x) {
          if b != b' ' {
            first = first.or(Some(y));
            last = Some(y);
          }
        }
      }
      ylimits.push(first.unwrap() as i32..last.unwrap() as i32 + 1);
    }
    Self { data: input.to_vec() }
  }
}
impl Board for CubeBoard {
  fn step(&self, pos: Point, heading: Heading) -> Option<(Point, Heading)> {
    fn face(p: Point) -> i32 {
      let fx = p.x / CUBE_SIZE;
      let fy = p.y / CUBE_SIZE;
      fx + fy * 3
    }
    fn face_pos(face: i32) -> Point {
      let fx = face % 3;
      let fy = face / 3;
      Point::new(CUBE_SIZE * fx, CUBE_SIZE * fy)
    }
    //
    //  +-----------+    +------+
    //  |           |    |      |
    //  |           v    v      |
    //  |         1111122222    |
    //  |         1111122222    |
    //  | +------>1111122222<-+ |
    //  | |       1111122222  | |
    //  | |       1111122222  | |
    //  | |       44444  ^    | |
    //  | |       44444  |    | |
    //  | |    +->44444<-+    | |
    //  | |    |  44444       | |
    //  | |    v  44444       | |
    //  | |  6666677777       | |
    //  | |  6666677777       | |
    //  | +->6666677777<------+ |
    //  |    6666677777         |
    //  |    6666677777         |
    //  |    99999  ^           |
    //  |    99999  |           |
    //  +--->99999<-+           |
    //       99999              |
    //       99999              |
    //         ^                |
    //         |                |
    //         +----------------+
    //
    //  ## 012  12
    //  #  345  4
    // ##  678 67
    // #   9AB 9
    //

    fn transform(
      pos: Point,
      heading: Heading,
      to: i32,
      target: i32,
      rot: i32,
    ) -> (Point, Heading) {
      fn rotate(pos: Point, rot: i32) -> Point {
        assert!(0 <= pos.x && pos.x < CUBE_SIZE);
        assert!(0 <= pos.y && pos.y < CUBE_SIZE);
        match rot.rem_euclid(4) {
          0 => pos,
          1 => Point::new(CUBE_SIZE - 1 - pos.y, pos.x),
          2 => Point::new(CUBE_SIZE - 1 - pos.x, CUBE_SIZE - 1 - pos.y),
          3 => Point::new(pos.y, CUBE_SIZE - 1 - pos.x),
          _ => unsafe { unreachable_unchecked() },
        }
      }
      let new_point = rotate(pos - face_pos(to), rot) + face_pos(target);
      let new_heading = heading.rot(rot);
      (new_point, new_heading)
    }

    let new = pos + heading.dir();
    let new = Point::new(
      new.x.rem_euclid(3 * CUBE_SIZE),
      new.y.rem_euclid(4 * CUBE_SIZE),
    );

    macro_rules! map {
        ($($from: literal -> $target: literal
           (via $to: literal)
           rot $rot: literal)+) => {
          match (face(pos), face(new)) {
            $(($from, $to) => {
              let res = transform(new, heading, $to, $target, $rot);
              assert_eq!(face(res.0), $target);
              res
            },)+
            (from, to) => {
              assert!([1, 2, 4, 6, 7, 9].contains(&from));
              assert!([1, 2, 4, 6, 7, 9].contains(&to));
              (new, heading)
            }
          }
        };
    }

    let (new, new_heading) = map!(
      0x1 -> 0x6 (via 0x0) rot 2
      0x6 -> 0x1 (via 0x8) rot 2
      0x1 -> 0x9 (via 0xA) rot 1
      0x9 -> 0x1 (via 0xB) rot -1
      0x2 -> 0x4 (via 0x5) rot 1
      0x4 -> 0x2 (via 0x5) rot -1
      0x2 -> 0x7 (via 0x0) rot 2
      0x7 -> 0x2 (via 0x8) rot 2
      0x2 -> 0x9 (via 0xB) rot 0
      0x9 -> 0x2 (via 0x0) rot 0
      0x4 -> 0x6 (via 0x3) rot -1
      0x6 -> 0x4 (via 0x3) rot 1
      0x7 -> 0x9 (via 0xA) rot 1
      0x9 -> 0x7 (via 0xA) rot -1
    );
    if self.data[new.y as usize][new.x as usize] == b'.' {
      Some((new, new_heading))
    } else {
      None
    }
  }
}

fn solve(instructions: &[Action], board: &impl Board) -> AocResult<i32> {
  let mut pos = Point::new(CUBE_SIZE, 0);
  let mut heading = Heading { heading: 0 };

  for &inst in instructions {
    match inst {
      Action::Forward(steps) => {
        for _ in 0..steps {
          if let Some((new_pos, new_heading)) = board.step(pos, heading) {
            pos = new_pos;
            heading = new_heading;
          } else {
            break;
          }
        }
      }
      Action::Left => {
        heading = heading.rot(-1);
      }
      Action::Right => {
        heading = heading.rot(1);
      }
    }
  }

  let row = 1 + pos.y;
  let col = 1 + pos.x;
  let res = row * 1000 + col * 4 + heading.value();
  Ok(res)
}

pub fn run(input: &[Vec<u8>]) -> AocResult<(i32, i32)> {
  let mut input = input.to_vec();
  let instructions = parse_instructions(input.pop().unwrap());
  input.pop();
  Ok((
    solve(&instructions, &WrapBoard::new(&input))?,
    solve(&instructions, &CubeBoard::new(&input))?,
  ))
}
