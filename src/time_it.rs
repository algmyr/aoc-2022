pub struct TimeIt {
  name: String,
  start: std::time::Instant,
}

impl TimeIt {
  pub fn new(name: &str) -> TimeIt {
    TimeIt {
      name: name.to_owned(),
      start: std::time::Instant::now(),
    }
  }
  pub fn tic(&mut self) { self.start = std::time::Instant::now(); }
  pub fn toc(&self) {
    let elapsed = self.start.elapsed();
    let us = elapsed.as_nanos() / 1000;
    let (ms, us) = (us / 1000, us % 1000);
    println!("{}: {:>4}.{:<03}ms", self.name, ms, us);
  }
}
