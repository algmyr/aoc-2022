use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::{Parse, ParseStream}, Result, parse_macro_input};

struct DaySpec {
  day: syn::LitInt,
  n_runs: syn::LitInt,
  p1: Option<syn::Lit>,
  p2: Option<syn::Lit>,
}

impl DaySpec {
  fn module_path(&self) -> syn::Path {
    let p = format!("solutions::day{:02}", self.day.base10_parse::<i32>().unwrap());
    let s: TokenStream = p.parse().unwrap();
    let path: syn::Path = syn::parse(s).unwrap();
    path
  }
}

impl Parse for DaySpec {
  fn parse(input: ParseStream) -> Result<Self> {
    if input.parse::<syn::Ident>()? != "day" { panic!("aaa"); }
    let day = input.parse::<syn::LitInt>()?;
    input.parse::<syn::Token!(:)>()?;
    let n_runs = input.parse::<syn::LitInt>()?;
    if input.parse::<syn::Ident>()? != "runs" { panic!("aaa"); }
    
    let (p1, p2) = if let Ok(_) = input.parse::<syn::Token!(,)>() {
      if input.parse::<syn::Ident>()? != "expected" { panic!("aaa"); }
      (Some(input.parse::<syn::Lit>()?), Some(input.parse::<syn::Lit>()?))
    } else {
      (None, None)
    };
    Ok(DaySpec { day, n_runs, p1, p2 })
  }
}

fn aoc_run(spec: &DaySpec) -> proc_macro2::TokenStream {
  let path = spec.module_path();
  let day = &spec.day;
  let n_runs = &spec.n_runs;

  let p1_check = spec.p1.as_ref().map(|p1| quote!(
    if res1 != #p1 {
      panic!("Wrong answer on day {} part 1. Expected {}, got {}",
             #day, #p1, res1);
    }
  ));
  let p2_check = spec.p2.as_ref().map(|p2| quote!(
    if res2 != #p2 {
      panic!("Wrong answer on day {} part 2. Expected {}, got {}",
             #day, #p2, res2);
    }
  ));

  quote!({
    let day = format!("{:02}", #day);
    let input_file = format!("inputs/{}input", day);

    let mut run_elapsed = vec![];
    let mut parse_elapsed = vec![];

    let mut run_once = || -> AocResult<_> {
      use #path as day;
      let t = std::time::Instant::now();
      let input = day::parse_input(&input_file)?;
      parse_elapsed.push(t.elapsed());
      let t = std::time::Instant::now();
      let (res1, res2) = day::run(&input)?;
      run_elapsed.push(t.elapsed());
      Ok((res1, res2))
    };

    for _ in 1usize..#n_runs {
      run_once()?;
    }
    let (res1, res2) = run_once()?;

    #p1_check
    #p2_check

    bench::RunResult {
      parse_elapsed,
      run_elapsed,
      part1_result: Box::new(res1),
      part2_result: Box::new(res2),
    }
  })
}

#[proc_macro]
pub fn aoc_bench_proc(stream: TokenStream) -> TokenStream {
  let spec = parse_macro_input!(stream as DaySpec);
  let day = &spec.day;
  let n_runs = &spec.n_runs;

  if n_runs.base10_digits() == "0" {
    quote!({
      let day = format!("{:02}", #day);

      (day, bench::RunResult { 
        parse_elapsed: vec![std::time::Duration::ZERO],
        run_elapsed: vec![std::time::Duration::ZERO],
        part1_result: Box::new("Day"),
        part2_result: Box::new("Skipped"),
      })
    }).into()
  } else {
    let run = aoc_run(&spec);
    quote!({
      let day = format!("{:02}", #day);
      let input_file = format!("inputs/{}input", day);

      println!("Timing day {} {} times...", day, #n_runs);
      let res = #run;

      (day, res)
    }).into()
  }
}
