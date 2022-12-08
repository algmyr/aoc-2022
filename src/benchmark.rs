// Rust macros are weird and fun
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
