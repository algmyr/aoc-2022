// Rust macros are weird and fun

use std::{time::Duration, fmt::Display};

pub struct RunResult {
  pub parse_elapsed: Vec<Duration>,
  pub run_elapsed: Vec<Duration>,
  pub part1_result: Box<dyn Display>,
  pub part2_result: Box<dyn Display>,
}

impl RunResult {
  pub fn avg_elapsed(&self) -> Duration {
    self.run_elapsed.iter().sum::<Duration>() / (self.run_elapsed.len() as u32)
  }
}

#[macro_export]
macro_rules! aoc_benchmark {
  ($_n_runs:expr, $_idx:expr, @outvec $_times: ident,) => {};

  ($n_runs:expr, $idx:expr, @outvec $results: ident, $head:path, $($tail:path,)*) => {
    let day = format!("{:02}", $idx);
    let input_file = format!("inputs/{}input", day);

    let res = aoc_run!($head, input_file, $n_runs);
    $results.push((day, res));

    aoc_benchmark!($n_runs, $idx + 1usize, @outvec $results, $($tail,)*);
  };

  (@n_runs $n_runs:expr, $($n:path),* $(,)?) => {{
    let mut results = vec![];
    aoc_benchmark!($n_runs, 1usize, @outvec results, $($n,)*);
    results
  }}
}

#[macro_export]
macro_rules! aoc_bench {
  (day $idx:literal : run $head:path | $n_runs:literal) => {{
    let day = format!("{:02}", $idx);
    let input_file = format!("inputs/{}input", day);

    if $n_runs > 0 {
      println!("Timing day {day} {} times...", $n_runs);
      let res = aoc_run!($head, input_file, $n_runs);
      (day, res)
    } else {
      (day, RunResult { 
        parse_elapsed: vec![std::time::Duration::ZERO],
        run_elapsed: vec![std::time::Duration::ZERO],
        part1_result: Box::new("Day"),
        part2_result: Box::new("Skipped"),
      })
    }
  }};
}

#[macro_export]
macro_rules! aoc_run_batch {
  ($module: path, $fname: expr, $n: expr) => {{
    let mut run_elapsed = vec![];
    let mut parse_elapsed = vec![];

    let mut run_once = || -> AocResult<_> {
      use $module as day;
      let input = day::parse_input(&$fname)?;
      Ok(day::run(&input)?)
    };

    let t = std::time::Instant::now();
    for _ in 1..$n {
      run_once()?;
    }
    let (res1, res2) = run_once()?;
    run_elapsed.push(t.elapsed()/($n));
    parse_elapsed.push(std::time::Duration::ZERO);

    RunResult {
      parse_elapsed,
      run_elapsed,
      part1_result: Box::new(res1),
      part2_result: Box::new(res2),
    }
  }};
}

#[macro_export]
macro_rules! aoc_run {
  ($module: path, $fname: expr, $n: expr) => {{
    let mut run_elapsed = vec![];
    let mut parse_elapsed = vec![];

    let mut run_once = || -> AocResult<_> {
      use $module as day;
      let t = std::time::Instant::now();
      let input = day::parse_input(&$fname)?;
      parse_elapsed.push(t.elapsed());
      let t = std::time::Instant::now();
      let (res1, res2) = day::run(&input)?;
      run_elapsed.push(t.elapsed());
      Ok((res1, res2))
    };

    for _ in 1usize..$n {
      run_once()?;
    }
    let (res1, res2) = run_once()?;

    RunResult {
      parse_elapsed,
      run_elapsed,
      part1_result: Box::new(res1),
      part2_result: Box::new(res2),
    }
  }};
}
