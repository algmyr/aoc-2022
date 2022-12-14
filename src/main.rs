use std::fmt::Display;
use std::time::Duration;

use aoc::error::AocResult;
use aoc::{aoc_benchmark, aoc_run, solutions, aoc_run_batch};
use itertools::Itertools;

struct RunResult {
  parse_elapsed: Vec<Duration>,
  run_elapsed: Vec<Duration>,
  part1_result: Box<dyn Display>,
  part2_result: Box<dyn Display>,
}

impl RunResult {
  fn avg_elapsed(&self) -> Duration {
    self.run_elapsed.iter().sum::<Duration>() / (self.run_elapsed.len() as u32)
  }
}

fn pretty_time(duration: Duration) -> String {
  let nanos = duration.as_nanos();
  let (t, unit) = if nanos > 1_000_000_000 {
    (nanos as f64 / 1e9, "s ")
  } else if nanos > 1_000_000 {
    (nanos as f64 / 1e6, "ms")
  } else if nanos > 1_000 {
    (nanos as f64 / 1e3, "us")
  } else {
    (nanos as f64 / 1e1, "ns")
  };

  format!("{t:7.3}{unit}")
}

fn run_all(n_runs: usize) -> AocResult<()> {
  let times = aoc_benchmark!(
    @n_runs n_runs,
    solutions::day01,
    solutions::day02,
    solutions::day03,
    solutions::day04,
    solutions::day05,
    solutions::day06,
    solutions::day07,
    solutions::day08,
    solutions::day09,
    solutions::day10,
    solutions::day11,
    solutions::day12,
    solutions::day13,
    solutions::day14,
    //solutions::day15,
    //solutions::day16,
    //solutions::day17,
    //solutions::day18,
    //solutions::day19,
    //solutions::day20,
    //solutions::day21,
    //solutions::day22,
    //solutions::day23,
    //solutions::day24,
    //solutions::day25,
  );

  fn make_row(
    parse_elapsed: Duration,
    run_elapsed: Duration,
    total: Duration,
    max: Duration,
  ) -> (String, String) {
    let fraction = run_elapsed.as_secs_f32() / total.as_secs_f32();
    let max_fraction = run_elapsed.as_secs_f32() / max.as_secs_f32();

    let width = 20.0;
    let x = (2.0 * width * max_fraction) as usize;
    let bar = "─".repeat(x / 2) + if x % 2 == 1 { "╴" } else { "" };

    let run_time = pretty_time(run_elapsed);
    let parse_time = pretty_time(parse_elapsed);
    (
      format!("{:9} {:9} {:6.2}%", parse_time, run_time, 100.0 * fraction),
      bar,
    )
  }

  fn average(times: Vec<Duration>) -> Duration {
    let n_outliers = times.len()/10;
    let n_keep = times.len() - 2 * n_outliers;
    times
      .into_iter()
      .sorted()
      .skip(n_outliers)
      .take(n_keep)
      .sum::<Duration>()
      / (n_keep as u32)
  }

  let avg_times = times
    .into_iter()
    .map(|(label, x)| (label, average(x.parse_elapsed), average(x.run_elapsed)))
    .collect_vec();

  let run_max: Duration = avg_times.iter().map(|(_, _parse, run)| *run).max().unwrap();
  let run_total: Duration = avg_times.iter().map(|(_, _parse, run)| run).sum();
  let parse_total: Duration = avg_times.iter().map(|(_, parse, _run)| parse).sum();

  println!("");
  println!("      {:^9} {:^9} ", "Parse", "Run");
  println!("    ╭─────────────────────────────╮");
  for (name, parse_elapsed, run_elapsed) in avg_times {
    let (contents, bar) = make_row(parse_elapsed, run_elapsed, run_total, run_max);
    println!("{:3} │ {} ├{}", name, contents, bar);
  }
  println!("    ├─────────────────────────────┤");
  let (contents, _) = make_row(parse_total, run_total, run_total, run_max);
  println!("Sum │ {} │", contents);
  println!("    ╰─────────────────────────────╯");
  println!("");
  Ok(())
}

fn main() -> AocResult<()> {
  let args: Vec<String> = std::env::args().collect();

  let n_runs = args.get(2).map(|s| s.parse().unwrap()).unwrap_or(1);

  if args[1].starts_with("bench") {
    run_all(n_runs)?;
  } else {
    let res = aoc_run_batch!(solutions::day14, args[1], n_runs as u32);
    println!("Part 1: {}", res.part1_result);
    println!("Part 2: {}", res.part2_result);
    println!("Elapsed: {}", pretty_time(res.avg_elapsed()));
  }
  Ok(())
}
