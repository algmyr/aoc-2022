use itertools::Itertools;

use crate::error::AocResult;
use crate::utils::read_all_nums_from_bytes;

#[derive(Debug)]
pub struct Blueprint {
  id: i32,
  ore_ore: i32,
  clay_ore: i32,
  obs_ore: i32,
  obs_clay: i32,
  geode_ore: i32,
  geode_obs: i32,
}

impl Blueprint {
  fn max_ore(&self) -> i32 {
    self
      .ore_ore
      .max(self.clay_ore)
      .max(self.obs_ore)
      .max(self.geode_ore)
  }
  fn max_clay(&self) -> i32 { self.obs_clay }
  fn max_obsidian(&self) -> i32 { self.geode_obs }

  fn try_buy_ore(&self, res: &Resources) -> Option<Resources> {
    if self.ore_ore <= res.ore {
      let mut new_res = *res;
      new_res.ore -= self.ore_ore;
      Some(new_res)
    } else {
      None
    }
  }

  fn try_buy_clay(&self, res: &Resources) -> Option<Resources> {
    if self.clay_ore <= res.ore {
      let mut new_res = *res;
      new_res.ore -= self.clay_ore;
      Some(new_res)
    } else {
      None
    }
  }

  fn try_buy_obsidian(&self, res: &Resources) -> Option<Resources> {
    if self.obs_ore <= res.ore && self.obs_clay <= res.clay {
      let mut new_res = *res;
      new_res.ore -= self.obs_ore;
      new_res.clay -= self.obs_clay;
      Some(new_res)
    } else {
      None
    }
  }

  fn try_buy_geode(&self, res: &Resources) -> Option<Resources> {
    if self.geode_ore <= res.ore && self.geode_obs <= res.obsidian {
      let mut new_res = *res;
      new_res.ore -= self.geode_ore;
      new_res.obsidian -= self.geode_obs;
      Some(new_res)
    } else {
      None
    }
  }
}

pub fn parse_input(fname: &str) -> AocResult<Vec<Blueprint>> {
  let b = std::fs::read(fname)?;

  let res = read_all_nums_from_bytes(&b)?
    .array_chunks()
    .map(
      |&[id, ore_ore, clay_ore, obs_ore, obs_clay, geode_ore, geode_clay]| Blueprint {
        id,
        ore_ore,
        clay_ore,
        obs_ore,
        obs_clay,
        geode_ore,
        geode_obs: geode_clay,
      },
    )
    .collect_vec();

  Ok(res)
}

#[derive(Debug, Clone, Copy)]
struct Resources {
  ore: i32,
  clay: i32,
  obsidian: i32,
  geode: i32,
}

impl Resources {
  fn new(ore: i32, clay: i32, obsidian: i32, geode: i32) -> Self {
    Self { ore, clay, obsidian, geode }
  }
}

#[derive(Debug, Clone, Copy)]
struct Bots {
  ore: i32,
  clay: i32,
  obsidian: i32,
  geode: i32,
}

impl Bots {
  fn new(ore: i32, clay: i32, obsidian: i32, geode: i32) -> Self {
    Self { ore, clay, obsidian, geode }
  }
}

fn update_resources(resources: &Resources, bots: &Bots) -> Resources {
  Resources {
    ore: resources.ore + bots.ore,
    clay: resources.clay + bots.clay,
    obsidian: resources.obsidian + bots.obsidian,
    geode: resources.geode + bots.geode,
  }
}

fn upper_bound_sim(
  blueprint: &Blueprint,
  lim: i32,
  turn: i32,
  resources: &Resources,
  bots: &Bots,
) -> i32 {
  let mut resources = *resources;
  let mut bots = *bots;
  for _ in turn..lim {
    // Ore robots are free.
    bots.ore += 1;

    // Buy clay.
    if blueprint.clay_ore <= resources.ore {
      resources.ore -= blueprint.clay_ore;
      bots.clay += 1;
    }
    // Buy obsidian.
    if blueprint.obs_clay <= resources.clay {
      resources.clay -= blueprint.obs_clay;
      bots.obsidian += 1;
    }
    // Buy geode.
    if blueprint.geode_obs <= resources.obsidian {
      resources.obsidian -= blueprint.geode_obs;
      bots.geode += 1;
    }

    // Do nothing.
    resources = update_resources(&resources, &bots);
  }
  resources.geode
}

fn do_blueprint(blueprint: &Blueprint, lim: i32) -> i32 {
  let mut stack = vec![(
    0,
    Resources::new(0, 0, 0, 0),
    Bots::new(1, 0, 0, 0),
    Bots::new(0, 0, 0, 0),
  )];
  let mut max_geodes = 0;
  while let Some((turn, resources, bots, mut banned)) = stack.pop() {
    if turn == lim {
      if resources.geode > max_geodes {
        max_geodes = resources.geode;
      }
      continue;
    }

    // Ditch useless states.
    if upper_bound_sim(blueprint, lim, turn, &resources, &bots) <= max_geodes {
      continue;
    }

    // Buy geode.
    if banned.geode == 0 {
      if let Some(new_res) = blueprint.try_buy_geode(&resources) {
        let new_res = update_resources(&new_res, &bots);
        stack.push((
          turn + 1,
          new_res,
          Bots { geode: bots.geode + 1, ..bots },
          Bots::new(0, 0, 0, 0),
        ));
        banned.geode = 1;
      }
    }
    // Buy obsidian.
    if banned.obsidian == 0 && bots.obsidian < blueprint.max_obsidian() {
      if let Some(new_res) = blueprint.try_buy_obsidian(&resources) {
        let new_res = update_resources(&new_res, &bots);
        stack.push((
          turn + 1,
          new_res,
          Bots { obsidian: bots.obsidian + 1, ..bots },
          Bots::new(0, 0, 0, 0),
        ));
        banned.obsidian = 1;
      }
    }
    // Buy clay.
    if banned.clay == 0 && bots.clay < blueprint.max_clay() {
      if let Some(new_res) = blueprint.try_buy_clay(&resources) {
        let new_res = update_resources(&new_res, &bots);
        stack.push((
          turn + 1,
          new_res,
          Bots { clay: bots.clay + 1, ..bots },
          Bots::new(0, 0, 0, 0),
        ));
        banned.clay = 1;
      }
    }
    // Buy ore.
    if banned.ore == 0 && bots.ore < blueprint.max_ore() {
      if let Some(new_res) = blueprint.try_buy_ore(&resources) {
        let new_res = update_resources(&new_res, &bots);
        stack.push((
          turn + 1,
          new_res,
          Bots { ore: bots.ore + 1, ..bots },
          Bots::new(0, 0, 0, 0),
        ));
        banned.ore = 1;
      }
    }

    // Do nothing.
    let resources = update_resources(&resources, &bots);
    stack.push((turn + 1, resources, bots, banned));
  }
  max_geodes
}

// If you buy something, buy as early as possible.
// Clay must be bought before obsidian.
// Obsidian must be bought before geode.
// The last relevant buy must be a geode.
// You must buy at least one geode.

fn part1(input: &[Blueprint]) -> AocResult<i32> {
  let mut res = 0;
  for blueprint in input.iter() {
    res += blueprint.id * do_blueprint(blueprint, 24);
  }
  Ok(res)
}

fn part2(input: &[Blueprint]) -> AocResult<i32> {
  let mut res = 1;
  for blueprint in input.iter().take(3) {
    res *= do_blueprint(blueprint, 32);
  }
  Ok(res)
}

pub fn run(input: &[Blueprint]) -> AocResult<(i32, i32)> {
  Ok((part1(input)?, part2(input)?))
}
