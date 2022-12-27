use ahash::AHashMap;
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
  conn: Vec<Vec<IxType>>,
  flow: Vec<i32>,
  map: AHashMap<&'a str, IxType>,
}

impl<'a> Graph<'a> {
  fn new(input: &[Valve]) -> Graph {
    let mut map = AHashMap::new();
    for (i, valve) in input.iter().sorted_by_key(|x| -x.rate).enumerate() {
      let key = &valve.name[..];
      map.insert(key, i as IxType);
    }
    let n = map.len();

    let ix = |name: &String| map[name.as_str()];

    let mut conn = vec![vec![]; n];
    let mut flow = vec![0; n];
    for valve in input {
      let src = ix(&valve.name);
      for dst in valve.dsts.iter().map(ix) {
        conn[src as usize].push(dst);
        flow[src as usize] = valve.rate;
      }
    }

    Graph { _input: input, conn, flow, map }
  }
}

type IxType = u8;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct State {
  cur: IxType,
  opened: u16,
}

impl State {
  fn is_opened(&self, i: IxType) -> bool { self.opened & (1 << i) > 0 }
  fn open(&mut self, i: IxType) { self.opened |= 1 << i; }
}

fn compute_best(
  input: &[Valve],
  released: Vec<AHashMap<State, i32>>,
  g: Graph,
  max_t: usize,
) -> Vec<i32> {
  // More complex logic to compensate for the longer sim.
  let nz = input.iter().filter(|&v| v.rate != 0).count();
  let mut best = vec![0; 1 << nz];
  let max_t2 = 26;
  for (state, res) in released[max_t2].iter() {
    // Total flow at cut-off.
    let mut i = 0;
    let mut n = state.opened;
    let mut total_flow = 0;
    while n != 0 {
      if n & 1 == 1 {
        total_flow += g.flow[i];
      }
      i += 1;
      n /= 2;
    }

    let compensated = *res - total_flow * (max_t - max_t2) as i32;
    let b = &mut best[state.opened as usize];
    *b = (*b).max(compensated);
  }
  best
}

fn distinct_subsets(n: usize) -> impl Iterator<Item = (usize, usize)> {
  // Iterates over distinct subsets of bits.
  // Way more noise than the loop version :<
  (0..n)
    .map(move |i| {
      let m = i ^ (n - 1);
      let n_subsets = 1<<m.count_ones();
      (0..n_subsets).scan(m, move |x, _| {
          let res = *x;
          *x = (*x - 1) & m;
          Some((i, res))
      })
    })
    .flatten()
}

fn solve(input: &[Valve]) -> AocResult<(i32, i32)> {
  let max_t = 30;

  let g = Graph::new(input);

  // Simulate.
  let mut released = vec![AHashMap::new(); max_t + 1];
  let start = State { cur: g.map["AA"], opened: 0 };
  released[0].insert(start, 0);

  for t in 1..released.len() {
    let states = released[t - 1].keys().cloned().collect_vec();
    for state in states {
      let cur = state.cur;
      let cur_release = released[t - 1][&state];

      if !state.is_opened(cur) {
        let will_release = g.flow[cur as usize] * (max_t - t) as i32;
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

      for &neigh in &g.conn[cur as usize] {
        let new_state = State { cur: neigh, opened: state.opened };
        let rel = released[t].entry(new_state).or_insert(-1);
        if *rel < cur_release {
          *rel = cur_release;
        }
      }
    }
  }

  let res1 = *released[30].values().max().unwrap_or(&-1);

  let best = compute_best(input, released, g, max_t);

  let res2 = distinct_subsets(best.len())
    .map(|(i, j)| best[i] + best[j])
    .max()
    .unwrap_or(-1);

  Ok((res1, res2))
}

pub fn run(input: &[Valve]) -> AocResult<(i32, i32)> { solve(input) }
