use std::fmt::Display;
use std::time::Duration;

use aoc::error::AocError;
use aoc::solutions;
use itertools::Itertools;

const N_RUNS: usize = 100;

// Rust macros are weird and fun
macro_rules! aoc_benchmark {
  (@step $_idx:expr, @outvec $_times: ident,) => {};

  (@step $idx:expr, @outvec $times: ident, $head:path, $($tail:path,)*) => {
    let day = format!("{:02}", $idx);
    let input_file = format!("inputs/{}input", day);
    $times.push((day, time_it!($head(&input_file)?, N_RUNS).elapsed,));

    aoc_benchmark!(@step $idx + 1usize, @outvec $times, $($tail,)*);
  };

  ($($n:path),* $(,)?) => {{
    let mut times = vec![];
    aoc_benchmark!(@step 1usize, @outvec times, $($n,)*);
    times
  }}
}

macro_rules! time_it {
  ($f: expr, $n: expr) => {{
    let mut times = vec![];

    let t = std::time::Instant::now();
    let (res1, res2) = $f;
    times.push(t.elapsed());

    for _ in 1..$n {
      let t = std::time::Instant::now();
      $f;
      times.push(t.elapsed());
    }
    RunResult {
      elapsed: times,
      part1_result: Box::new(res1),
      part2_result: Box::new(res2),
    }
  }};
}

struct RunResult {
  elapsed: Vec<Duration>,
  part1_result: Box<dyn Display>,
  part2_result: Box<dyn Display>,
}

impl RunResult {
  fn avg_elapsed(&self) -> Duration {
    self.elapsed.iter().sum::<Duration>() / (self.elapsed.len() as u32)
  }
}

fn run_all() -> Result<(), AocError> {
  let times = aoc_benchmark!(
    solutions::day01::run,
    solutions::day02::run,
    //solutions::day03::run,
    //solutions::day04::run,
    //solutions::day05::run,
    //solutions::day06::run,
    //solutions::day07::run,
    //solutions::day08::run,
    //solutions::day09::run,
    //solutions::day10::run,
    //solutions::day11::run,
    //solutions::day12::run,
    //solutions::day13::run,
    //solutions::day14::run,
    //solutions::day15::run,
    //solutions::day16::run,
    //solutions::day17::run,
    //solutions::day18::run,
    //solutions::day19::run,
    //solutions::day20::run,
    //solutions::day21::run,
    //solutions::day22::run,
    //solutions::day23::run,
    //solutions::day24::run,
    //solutions::day25::run,
  );

  fn print_row(name: &str, elapsed: Duration, total: Duration, max: Duration) {
    let width = 20.0;
    let fraction = elapsed.as_secs_f32() / total.as_secs_f32();
    let max_fraction = elapsed.as_secs_f32() / max.as_secs_f32();

    let ms = elapsed.as_secs_f64() * 1e3;
    if fraction < 1.0 {
      let x = (2.0 * width * max_fraction) as usize;
      let bar = "─".repeat(x / 2) + if x % 2 == 1 { "╴" } else { "" };
      println!(
        "{:>3} │{:9.3}ms {:6.2}% ├{}",
        name,
        ms,
        100.0 * fraction,
        bar,
      );
    } else {
      println!("{:>3} │{:9.3}ms {:6.2}% │", name, ms, 100.0 * fraction,);
    }
  }

  fn average(times: Vec<Duration>) -> Duration {
    let n_outliers = 0; // (times.len() + 9)/10; // Remove top/bottom 10%
    let n_keep = times.len() - 2*n_outliers;
    times.into_iter().sorted().skip(n_outliers).take(n_keep).sum::<Duration>()/(n_keep as u32)
  }

  let times = times
    .into_iter()
    .map(|(label, x)| (label, average(x)))
    .collect_vec();

  println!("");
  println!("    ╭────────────────────╮");
  let total: Duration = times.iter().map(|(_, e)| e).sum();
  let max: Duration = times.iter().map(|(_, e)| e).copied().max().unwrap();
  for (name, elapsed) in times {
    print_row(&name, elapsed, total, max);
  }
  println!("    ├────────────────────┤");
  print_row("Sum", total, total, max);
  println!("    ╰────────────────────╯");

  println!("");
  Ok(())
}

fn main() -> Result<(), AocError> {
  let args: Vec<String> = std::env::args().collect();
  if args[1] == "benchmark" {
    run_all()?;
  } else {
    let res = time_it!(solutions::day02::run(&args[1])?, 1);
    println!("Part 1: {}", res.part1_result);
    println!("Part 2: {}", res.part2_result);
    println!("Elapsed: {:.3}ms", res.avg_elapsed().as_secs_f64() * 1e3);
  }
  Ok(())
}
