use std::fmt::Display;
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

fn truncate<const M: usize, T: Clone + Default, const N: usize>(arr: [T; N]) -> [T; M] {
  let mut dst: [T; M] = std::array::from_fn(|_| Default::default());
  dst.clone_from_slice(&arr[..M]);
  dst
}

fn run_all() -> AocResult<()> {
  let times = vec![
    aoc_bench_proc!(day  1: 1000 runs, expected 71471 211189),
    aoc_bench_proc!(day  2: 1000 runs, expected 10310 14859),
    aoc_bench_proc!(day  3: 1000 runs, expected 7831 2683),
    aoc_bench_proc!(day  4: 1000 runs, expected 464 770),
    aoc_bench_proc!(day  5: 1000 runs, expected "VJSFHWGFT" "LCTQFBVZV"),
    aoc_bench_proc!(day  6: 1000 runs, expected 1578 2178),
    aoc_bench_proc!(day  7: 1000 runs, expected 1543140 1117448),
    aoc_bench_proc!(day  8: 1000 runs, expected 1538 496125),
    aoc_bench_proc!(day  9: 1000 runs, expected 6339 2541),
    aoc_bench_proc!(day 10: 1000 runs), //expected 13860 "RZHFGJCB"),
    aoc_bench_proc!(day 11: 1000 runs, expected 110220 19457438264_i64),
    aoc_bench_proc!(day 12: 1000 runs, expected 456 454),
    aoc_bench_proc!(day 13: 1000 runs, expected 6420 22000),
    aoc_bench_proc!(day 14: 1000 runs, expected 838 27539),
    aoc_bench_proc!(day 15: 1000 runs, expected 4907780 13639962836448_i64),
    aoc_bench_proc!(day 16:   10 runs, expected 1737 2216),
    aoc_bench_proc!(day 17: 1000 runs, expected 3071 1523615160362_i64),
    aoc_bench_proc!(day 18: 1000 runs, expected 4450 2564),
    aoc_bench_proc!(day 19:  100 runs, expected 1565 10672),
    aoc_bench_proc!(day 20:  100 runs, expected 11616 9937909178485_i64),
    aoc_bench_proc!(day 21: 1000 runs, expected 256997859093114.0 3952288690726_i64),
    aoc_bench_proc!(day 22: 1000 runs, expected 30552 184106),
    aoc_bench_proc!(day 23:   30 runs, expected 3987 938),
    aoc_bench_proc!(day 24:   30 runs, expected 242 720),
    aoc_bench_proc!(day 25: 1000 runs, expected "2-20=01--0=0=0=2-120" "Done!"),
  ];

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

  fn make_row<const N: usize>(
    prefix: &str,
    parse_elapsed: Duration,
    run_elapsed: Duration,
    total: Duration,
    max: Duration,
  ) -> [String; N] {
    let fraction = run_elapsed.as_secs_f32() / total.as_secs_f32();
    let max_fraction = run_elapsed.as_secs_f32() / max.as_secs_f32();

    let width = 20.0;
    let x = (2.0 * width * max_fraction) as usize;
    let bar = "─".repeat(x / 2) + if x % 2 == 1 { "╴" } else { "" };
    truncate([
      prefix.to_string(),
      pretty_time(parse_elapsed),
      pretty_time(run_elapsed),
      format!("{:.2}%", 100.0 * fraction),
      bar,
    ])
  }

  let run_max = avg_times.iter().map(|(_, _parse, run)| *run).max().unwrap();
  let run_total = avg_times.iter().map(|(_, _parse, run)| run).sum();
  let parse_total = avg_times.iter().map(|(_, parse, _run)| parse).sum();

  let mut table = Table::new();
  use Row::*;
  table.push(Row::header("", "Parse", "Run"));
  for (name, parse_elapsed, run_elapsed) in avg_times {
    table.push(Data(make_row(
      &name,
      parse_elapsed,
      run_elapsed,
      run_total,
      run_max,
    )));
  }
  table.push(Summary(make_row(
    "Sum",
    parse_total,
    run_total,
    run_total,
    run_max,
  )));

  println!("\n{table}\n");
  Ok(())
}

enum Row {
  Header([String; 3]),
  Data([String; 5]),
  Summary([String; 4]),
}

impl Row {
  fn header(a: &str, b: &str, c: &str) -> Self {
    Self::Header([a.to_string(), b.to_string(), c.to_string()])
  }
}

struct Table {
  data: Vec<Row>,
}

impl Table {
  fn new() -> Self { Table { data: vec![] } }
  fn push(&mut self, s: Row) { self.data.push(s); }
}

impl Display for Table {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    fn fmt(row: &Row, widths: &[usize; 4]) -> String {
      use Row::*;
      let [w0, w1, w2, w3] = widths;
      match row {
        Header([prefix, parse, run]) => {
          format!("{:w0$}   {:^w1$} {:^w2$}", prefix, parse, run)
        }
        Data([prefix, parse, run, perc, bar]) => {
          let content = format!("{parse:>w1$} {run:>w2$} {perc:>w3$}");
          if run.ends_with("us") || run.ends_with("ns") {
            format!("{prefix:w0$} │ \x1b[92m{content}\x1b[0m ├{bar}")
          } else {
            format!("{prefix:w0$} │ \x1b[91m{content}\x1b[0m ├{bar}")
          }
        },
        Summary([prefix, parse, run, perc]) => format!(
          "{:w0$} │ {:>w1$} {:>w2$} {:>w3$} │",
          prefix, parse, run, perc
        ),
      }
    }
    fn line(chars: [&str; 3], widths: &[usize; 4]) -> String {
      let [w0, w1, w2, w3] = widths;
      " ".repeat(w0 + 1) + chars[0] + &chars[1].repeat(4 + w1 + w2 + w3) + chars[2]
    }
    let header = self.data.first().unwrap();
    let summary = self.data.last().unwrap();
    let data = &self.data[1..self.data.len() - 1];
    let widths = [3, 9, 9, 7];
    let mut lines = vec![];
    lines.push(fmt(header, &widths));
    lines.push(line(["╭", "─", "╮"], &widths));
    lines.extend(data.iter().map(|s| fmt(s, &widths)));
    lines.push(line(["├", "─", "┤"], &widths));
    lines.push(fmt(summary, &widths));
    lines.push(line(["╰", "─", "╯"], &widths));
    write!(f, "{}", lines.join("\n"))
  }
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
