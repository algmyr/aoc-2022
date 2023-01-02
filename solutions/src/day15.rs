use itertools::{iproduct, Itertools};

use error::AocResult;
use crate::utils::read_all_signed_nums;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
  x: i32,
  y: i32,
}

impl Point {
  fn new(x: i32, y: i32) -> Self { Point { x, y } }
  fn dist(&self, other: Self) -> u32 {
    self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
  }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Circle {
  center: Point,
  rad: u32,
}

impl Circle {
  fn new(sensor: Point, beacon: Point) -> Self {
    Circle { center: sensor, rad: sensor.dist(beacon) }
  }
  fn x(&self) -> i32 { self.center.x }
  fn y(&self) -> i32 { self.center.y }
  fn first_at(&self, y: i32) -> Option<i32> {
    let dy = self.y().abs_diff(y);
    if dy <= self.rad {
      let dx = self.rad - dy;
      Some(self.x() - dx as i32)
    } else {
      None
    }
  }
  fn last_at(&self, y: i32) -> Option<i32> {
    let dy = self.y().abs_diff(y);
    if dy <= self.rad {
      let dx = self.rad - dy;
      Some(self.x() + dx as i32)
    } else {
      None
    }
  }
}

pub fn parse_input(fname: &str) -> AocResult<(Vec<Circle>, Vec<Point>)> {
  let mut circles = vec![];
  let mut beacons = vec![];
  for &[sx, sy, bx, by] in  read_all_signed_nums(fname)?.array_chunks() {
    let s = Point::new(sx, sy);
    let b = Point::new(bx, by);
    circles.push(Circle::new(s, b));
    beacons.push(b);
  }
  Ok((circles, beacons))
}

enum Event {
  In(i32),
  Out(i32),
}

fn part1(input: &[Circle], beacons: &[Point]) -> AocResult<i32> {
  let target = 2000000;

  let mut events = vec![];
  for c in input {
    if let Some(first) = c.first_at(target) {
      events.push(Event::In(first));
    }
    if let Some(last) = c.last_at(target) {
      events.push(Event::Out(last + 1));
    }
  }

  events.sort_by_key(|e| match e {
    Event::In(x) => *x,
    Event::Out(x) => *x,
  });

  let mut res = 0;
  let mut last = 0;
  let mut d = 0;
  for e in events {
    match e {
      Event::In(x) => {
        if d == 0 {
          last = x;
        }
        d += 1;
      }
      Event::Out(x) => {
        d -= 1;
        if d == 0 {
          res += x - last;
        }
      }
    }
  }

  res -= beacons.iter().filter(|b| b.y == target).unique().count() as i32;
  Ok(res)
}

// The point we are looking for must be at edges of diamonds.
// Where 4 of them are close. 2 / lines and 2 \ lines
fn part2(input: &[Circle], _: &[Point]) -> AocResult<i64> {
  // Get x+y = const lines
  let mut offs_up = vec![];
  for c in input {
    offs_up.push(c.x() + c.y() - c.rad as i32);
    offs_up.push(c.x() + c.y() + c.rad as i32);
  }
  offs_up.sort_unstable();
  let mut close_up = vec![];
  for [a, b] in offs_up.array_windows() {
    if b - a <= 4 {
      close_up.push((a + 1, b - 1));
    }
  }

  // Get -x+y = const lines
  let mut offs_down = vec![];
  for c in input {
    offs_down.push(-c.x() + c.y() - c.rad as i32);
    offs_down.push(-c.x() + c.y() + c.rad as i32);
  }
  offs_down.sort_unstable();
  let mut close_down = vec![];
  for [a, b] in offs_down.array_windows() {
    if b - a <= 4 {
      close_down.push((a + 1, b - 1));
    }
  }

  for ((up_low, up_high), (down_low, down_high)) in
    iproduct!(close_up.into_iter(), close_down.into_iter())
  {
    for up in up_low..=up_high {
      for down in down_low..=down_high {
        //  x + y  == up
        // -x + y == down
        let x = (up - down) / 2;
        let y = (up + down) / 2;
        let p = Point::new(x, y);

        let mut ok = true;
        for c in input {
          if c.center.dist(p) <= c.rad {
            ok = false;
          }
        }
        if ok {
          return Ok(x as i64 * 4000000 + y as i64);
        }
      }
    }
  }

  Ok(-1)
}

pub fn run(input: &(Vec<Circle>, Vec<Point>)) -> AocResult<(i32, i64)> {
  let (input, beacons) = input;
  Ok((part1(input, beacons)?, part2(input, beacons)?))
}
