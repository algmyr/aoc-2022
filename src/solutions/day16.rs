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
    for (i, valve) in input.iter().sorted_by_key(|x| -x.rate).enumerate() {
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

fn part2_better(input: &[Valve]) -> AocResult<i32> {
  let max_t = 30;

  let g = Graph::new(input);

  let mut released = vec![HashMap::new(); max_t+1];
  let start = State { cur: g.map["AA"], opened: 0 };
  released[0].insert(start, 0);

  for t in 1..released.len() {
    let states = released[t - 1].keys().cloned().collect_vec();
    for state in states {
      let cur = state.cur;
      let cur_release = released[t - 1][&state];

      if !state.is_opened(cur) {
        let will_release = g.flow[cur] * (max_t - t) as i32;
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

  let nz = input.iter().filter(|&v| v.rate != 0).count();
  let mut best = vec![0; 1 << nz];
  let max_t2 = 26;
  for (state, res) in released[max_t2].iter() {
    let mut i = 0;
    let mut n = state.opened;
    let mut total_flow = 0;
    while n != 0 {
      if n&1 == 1 {
        total_flow += g.flow[i];
      }
      i += 1;
      n /= 2;
    }
    best[start.opened as usize] = best[start.opened as usize].max(*res - total_flow*(max_t - max_t2) as i32);
  }

  let mut mx = 0;
  for i in 0..best.len() {
    for j in 0..best.len() {
      if i&j == 0 {
        mx = mx.max(best[i]+best[j]);
      }
    }
  }
  Ok(mx)
}

pub fn run(input: &[Valve]) -> AocResult<(i32, i32)> {
  Ok((part1(input)?, part2_better(input)?))
}
