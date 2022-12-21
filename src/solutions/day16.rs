use std::collections::HashMap;

use bstr::ByteSlice;
use itertools::Itertools;

use crate::error::AocResult;

#[derive(Debug)]
pub struct Valve {
  name: String,
  rate: i32,
  dsts: Vec<String>,
}

pub fn parse_input(fname: &str) -> AocResult<Vec<Valve>> {
  let b = std::fs::read(fname)?;
  Ok(
    b.to_str_lossy()
      .lines()
      .map(|s| {
        let v = s.split(" ").collect_vec();
        let src = v[1];
        let rate = v[4]
          .strip_prefix("rate=")
          .unwrap()
          .strip_suffix(";")
          .unwrap();
        let dsts = &v[9..];

        Valve {
          name: src.to_string(),
          rate: rate.parse().unwrap(),
          dsts: dsts
            .iter()
            .map(|x| x.trim_matches(',').to_string())
            .collect_vec(),
        }
      })
      .collect_vec(),
  )
}

struct Graph<'a> {
  _input: &'a [Valve],
  conn: Vec<Vec<usize>>,
  flow: Vec<i32>,
  map: HashMap<&'a str, usize>,
}

impl<'a> Graph<'a> {
  fn new(input: &[Valve]) -> Graph {
    let mut map = HashMap::new();
    for (i, valve) in input.iter().enumerate() {
      let key = &valve.name[..];
      map.insert(key, i);
    }
    let n = map.len();

    let ix = |name: &String| map[name.as_str()];

    let mut conn = vec![vec![]; n];
    let mut flow = vec![0; n];
    for valve in input {
      let src = ix(&valve.name);
      for dst in valve.dsts.iter().map(ix) {
        conn[src].push(dst);
        flow[src] = valve.rate;
      }
    }

    Graph { _input: input, conn, flow, map }
  }
}

//fn bfs(start: usize, conn: &Vec<Vec<usize>>) -> Vec<i32> {
//  let mut queue = VecDeque::new();
//  queue.push_back((0, start));
//  let mut distances = vec![-1; conn.len()];
//  while let Some((d, cur)) = queue.pop_front() {
//    if distances[cur] != -1 {
//      continue;
//    }
//    distances[cur] = d;

//    for neigh in &conn[cur] {
//      queue.push_back((d+1, *neigh));
//    }
//  }

//  distances
//}

//let closest = (0..n).map(|i| bfs(i, &conn)).collect_vec();

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct State {
  cur: usize,
  opened: u64,
}

impl State {
  fn is_opened(&self, i: usize) -> bool { self.opened & (1 << i) > 0 }
  fn open(&mut self, i: usize) { self.opened |= 1 << i; }
}

fn part1(input: &[Valve]) -> AocResult<i32> {
  let g = Graph::new(input);

  let mut released = vec![HashMap::new(); 31];
  let start = State { cur: g.map["AA"], opened: 0 };
  released[0].insert(start, 0);

  for t in 1..released.len() {
    let states = released[t - 1].keys().cloned().collect_vec();
    for state in states {
      let cur = state.cur;
      let cur_release = released[t - 1][&state];

      if !state.is_opened(cur) {
        let will_release = g.flow[cur] * (30 - t as i32);
        if will_release > 0 {
          let mut new_state = state;
          new_state.open(cur);

          let new = cur_release + will_release;
          let val = released[t].entry(new_state).or_insert(-1);
          if *val < new {
            *val = new;
          }
        }
      }

      for &neigh in &g.conn[cur] {
        let new_state = State { cur: neigh, opened: state.opened };
        let rel = released[t].entry(new_state).or_insert(-1);
        if *rel < cur_release {
          *rel = cur_release;
        }
      }
    }
  }

  let res = *released[30].values().max().unwrap_or(&-1);
  Ok(res)
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct DuoState {
  cur0: usize,
  cur1: usize,
  opened: u64,
}

impl DuoState {
  fn is_opened(&self, i: usize) -> bool { self.opened & (1 << i) > 0 }
  fn open(&mut self, i: usize) { self.opened |= 1 << i; }
  fn cur(&self, time: usize) -> usize {
    if time % 2 == 0 {
      self.cur0
    } else {
      self.cur1
    }
  }
  fn cur_mut(&mut self, time: usize) -> &mut usize {
    if time % 2 == 0 {
      &mut self.cur0
    } else {
      &mut self.cur1
    }
  }
}

//fn part2_bleh(input: &[Valve]) -> AocResult<i32> {
//  let max_t = 26;
//
//  let g = Graph::new(input);
//
//  let n_openable = g.flow.iter().filter(|&&x| x > 0).count() as u32;
//
//  let mut released = HashMap::new();
//  let aa = g.map["AA"];
//  let start = DuoState { time: 0, cur0: aa, cur1: aa, opened: 0 };
//  released.insert(start, 0);
//
//  let mut best = 0;
//  let mut queue = VecDeque::new();
//  queue.push_back(start);
//  while let Some(state) = queue.pop_front() {
//    let cur = state.cur();
//    let cur_release = released[&state];
//
//    if state.opened.count_ones() == n_openable {
//      best = best.max(cur_release);
//      continue;
//    }
//    if state.time >= max_t*2 {
//      best = best.max(cur_release);
//      continue;
//    }
//    if !state.is_opened(cur) {
//      let will_release = g.flow[cur] * (max_t - state.time/2 - 1);
//      if will_release > 0 {
//        let mut new_state = state;
//        new_state.time += 1;
//        new_state.open(cur);
//
//        let new = cur_release + will_release;
//        let val = released.entry(new_state).or_insert(-1);
//        if *val < new {
//          *val = new;
//          queue.push_back(new_state);
//        }
//      }
//    }
//
//    for &neigh in &g.conn[cur] {
//      let mut new_state = state;
//      *new_state.cur_mut() = neigh;
//      new_state.time += 1;
//      let rel = released.entry(new_state).or_insert(-1);
//      if *rel < cur_release {
//        *rel = cur_release;
//        queue.push_back(new_state);
//      }
//    }
//  }
//
//  Ok(best)
//}

fn part2(input: &[Valve]) -> AocResult<i32> {
  let max_t = 26;
  let g = Graph::new(input);

  let mut released = vec![HashMap::new(); 2 * max_t + 1];
  let aa = g.map["AA"];
  let start = DuoState { cur0: aa, cur1: aa, opened: 0 };
  released[0].insert(start, 0);

  let total = g.flow.iter().sum::<i32>();

  for t in 1..released.len() {
    let t_remaining = (max_t - (t + 1) / 2) as i32;
    println!("t: {t}");
    let max = *released[t - 1].values().max().unwrap();
    let states = released[t - 1]
      .iter()
      .filter(|&(_, val)| val + total * t_remaining >= max)
      .map(|x| x.0)
      .cloned()
      .collect_vec();
    for state in states {
      let cur = state.cur(t);
      let cur_release = released[t - 1][&state];

      if !state.is_opened(cur) {
        let will_release = g.flow[cur] * t_remaining;
        if will_release > 0 {
          let mut new_state = state;
          new_state.open(cur);

          let new = cur_release + will_release;
          let val = released[t].entry(new_state).or_insert(-1);
          if *val < new {
            *val = new;
          }
        }
      }

      for &neigh in &g.conn[cur] {
        let mut new_state = state;
        *new_state.cur_mut(t) = neigh;
        let rel = released[t].entry(new_state).or_insert(-1);
        if *rel < cur_release {
          *rel = cur_release;
        }
      }
    }
  }

  let res = *released[2 * max_t].values().max().unwrap_or(&-1);
  Ok(res)
}

pub fn run(input: &[Valve]) -> AocResult<(i32, i32)> {
  Ok((part1(input)?, part2(input)?))
}
