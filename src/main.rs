use std::time::Duration;

use aoc::benchmark::RunResult;
use aoc::error::AocResult;
use aoc::{aoc_bench, aoc_run, aoc_run_batch, solutions};
use itertools::Itertools;

fn pretty_time(duration: Duration) -> String {
  if duration.is_zero() {
    return "---------".to_string();
  }
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

fn run_all() -> AocResult<()> {
  let times = vec![
    aoc_bench!(day  1: run solutions::day01 | 1000),
    aoc_bench!(day  2: run solutions::day02 | 1000),
    aoc_bench!(day  3: run solutions::day03 | 1000),
    aoc_bench!(day  4: run solutions::day04 | 1000),
    aoc_bench!(day  5: run solutions::day05 | 1000),
    aoc_bench!(day  6: run solutions::day06 | 1000),
    aoc_bench!(day  7: run solutions::day07 | 1000),
    aoc_bench!(day  8: run solutions::day08 | 1000),
    aoc_bench!(day  9: run solutions::day09 | 1000),
    aoc_bench!(day 10: run solutions::day10 | 1000),
    aoc_bench!(day 11: run solutions::day11 | 1000),
    aoc_bench!(day 12: run solutions::day12 | 1000),
    aoc_bench!(day 13: run solutions::day13 | 1000),
    aoc_bench!(day 14: run solutions::day14 | 1000),
    aoc_bench!(day 15: run solutions::day15 |    1),
    aoc_bench!(day 16: run solutions::day16 |    0),
    aoc_bench!(day 17: run solutions::day17 | 1000),
    aoc_bench!(day 18: run solutions::day18 |    1),
    aoc_bench!(day 19: run solutions::day19 |    0),
    aoc_bench!(day 20: run solutions::day20 |   10),
    aoc_bench!(day 21: run solutions::day21 | 1000),
    aoc_bench!(day 22: run solutions::day22 | 1000),
    //aoc_bench!(day 23: run solutions::day23 | 1000),
    //aoc_bench!(day 24: run solutions::day24 | 1000),
    //aoc_bench!(day 25: run solutions::day25 | 1000),
  ];

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
    let n_outliers = times.len() / 10;
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
    run_all()?;
  } else {
    let res = aoc_run_batch!(solutions::day22, args[1], n_runs as u32);
    println!("Part 1: {}", res.part1_result);
    println!("Part 2: {}", res.part2_result);
    println!("Elapsed: {}", pretty_time(res.avg_elapsed()));
  }
  Ok(())
}
