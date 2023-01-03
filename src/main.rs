use std::time::Duration;

use bench::aoc_run_batch;
use bench_proc::aoc_bench_proc;
use error::AocResult;
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
    aoc_bench_proc!(day  1: 100 runs, expected 71471 211189),
    aoc_bench_proc!(day  2: 100 runs, expected 10310 14859),
    aoc_bench_proc!(day  3: 100 runs, expected 7831 2683),
    aoc_bench_proc!(day  4: 100 runs, expected 464 770),
    aoc_bench_proc!(day  5: 100 runs, expected "VJSFHWGFT" "LCTQFBVZV"),
    aoc_bench_proc!(day  6: 100 runs, expected 1578 2178),
    aoc_bench_proc!(day  7: 100 runs, expected 1543140 1117448),
    aoc_bench_proc!(day  8: 100 runs, expected 1538 496125),
    aoc_bench_proc!(day  9: 100 runs, expected 6339 2541),
    aoc_bench_proc!(day 10: 100 runs), //expected 13860 "RZHFGJCB"),
    aoc_bench_proc!(day 11: 100 runs, expected 110220 19457438264_i64),
    aoc_bench_proc!(day 12: 100 runs, expected 456 454),
    aoc_bench_proc!(day 13: 100 runs, expected 6420 22000),
    aoc_bench_proc!(day 14: 100 runs, expected 838 27539),
    aoc_bench_proc!(day 15: 100 runs, expected 4907780 13639962836448_i64),
    aoc_bench_proc!(day 16: 1 runs, expected 1737 2216),
    aoc_bench_proc!(day 17: 100 runs, expected 3071 1523615160362_i64),
    aoc_bench_proc!(day 18: 100 runs, expected 4450 2564),
    aoc_bench_proc!(day 19:  10 runs, expected 1565 10672),
    aoc_bench_proc!(day 20:  10 runs, expected 11616 9937909178485_i64),
    aoc_bench_proc!(day 21: 100 runs, expected 256997859093114.0 3952288690726_i64),
    aoc_bench_proc!(day 22: 100 runs, expected 30552 184106),
    aoc_bench_proc!(day 23: 3 runs, expected 3987 938),
    aoc_bench_proc!(day 24: 3 runs, expected 242 720),
    aoc_bench_proc!(day 25: 100 runs, expected "2-20=01--0=0=0=2-120" "Done!"),
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
    let res = aoc_run_batch!(solutions::day17, args[1], n_runs as u32);
    println!("Part 1: {}", res.part1_result);
    println!("Part 2: {}", res.part2_result);
    println!("Elapsed: {}", pretty_time(res.avg_elapsed()));
  }
  Ok(())
}
