use itertools::Itertools;

use crate::error::AocResult;

#[derive(Debug)]
struct Monkey<T> {
  items: Vec<T>,
  op: fn(T) -> T,
  target: fn(T) -> usize,
  div: T,
}

macro_rules! monkey {
  (
    $type: ty,
    $($items:literal),*;
    $op0: tt
    $op2: tt
    $op3: tt
    $op4: tt,
    $div: literal,
    $true_dst: literal,
    $false_dst: literal
  ) => {
    Monkey::<$type> {
      items: vec![$($items),*],
      op: |$op2| { let $op0 = $op2 $op3 $op4; $op0 },
      target: |obj| if obj % $div == 0 { $true_dst } else { $false_dst },
      div: $div,
    }
  };
  (
    $type: ty,
    $(
    Monkey $i: literal:
      Starting items: $($items:literal),*
      Operation: $op0: tt = $op2: tt $op3: tt $op4: tt
      Test: divisible by $div: literal
        If true: throw to monkey $true_dst: literal
        If false: throw to monkey $false_dst: literal
  )+) => {
    [$(monkey!($type, $($items),*; $op0 $op2 $op3 $op4, $div, $true_dst, $false_dst),)+]
  };
}

macro_rules! monkeys {
  (input with $type: ty) => {
    monkey!($type,
      Monkey 0:
        Starting items: 53, 89, 62, 57, 74, 51, 83, 97
        Operation: new = old * 3
        Test: divisible by 13
          If true: throw to monkey 1
          If false: throw to monkey 5

      Monkey 1:
        Starting items: 85, 94, 97, 92, 56
        Operation: new = old + 2
        Test: divisible by 19
          If true: throw to monkey 5
          If false: throw to monkey 2

      Monkey 2:
        Starting items: 86, 82, 82
        Operation: new = old + 1
        Test: divisible by 11
          If true: throw to monkey 3
          If false: throw to monkey 4

      Monkey 3:
        Starting items: 94, 68
        Operation: new = old + 5
        Test: divisible by 17
          If true: throw to monkey 7
          If false: throw to monkey 6

      Monkey 4:
        Starting items: 83, 62, 74, 58, 96, 68, 85
        Operation: new = old + 4
        Test: divisible by 3
          If true: throw to monkey 3
          If false: throw to monkey 6

      Monkey 5:
        Starting items: 50, 68, 95, 82
        Operation: new = old + 8
        Test: divisible by 7
          If true: throw to monkey 2
          If false: throw to monkey 4

      Monkey 6:
        Starting items: 75
        Operation: new = old * 7
        Test: divisible by 5
          If true: throw to monkey 7
          If false: throw to monkey 0

      Monkey 7:
        Starting items: 92, 52, 85, 89, 68, 82
        Operation: new = old * old
        Test: divisible by 2
          If true: throw to monkey 0
          If false: throw to monkey 1
    )
  };
  (sample with $type: ty) => {
    monkey!($type,
      Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3

      Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
          If true: throw to monkey 2
          If false: throw to monkey 0

      Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
          If true: throw to monkey 1
          If false: throw to monkey 3

      Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
          If true: throw to monkey 0
          If false: throw to monkey 1
    )
  };
}

pub fn parse_input(_fname: &str) -> AocResult<()> { Ok(()) }

fn part1(_: &()) -> AocResult<i32> {
  let mut monkeys = monkeys!(input with i32);

  let mut counts = vec![0; monkeys.len()]; // Can't use array here with monkeys.len()?

  for _ in 0..20 {
    for i in 0..monkeys.len() {
      let (op, target, items) = {
        let monkey = &mut monkeys[i];
        (
          monkey.op,
          monkey.target,
          monkey.items.drain(..).collect_vec(),
        )
      };
      counts[i] += items.len() as i32;
      for item in items {
        let item = op(item);
        let item = item / 3;
        let target = target(item);
        monkeys[target].items.push(item)
      }
    }
  }

  let res = counts.into_iter().sorted().rev().take(2).product();
  Ok(res)
}

fn part2(_: &()) -> AocResult<i64> {
  let mut monkeys = monkeys!(input with i64);

  let modulo: i64 = monkeys.iter().map(|m| m.div).unique().product();

  let mut counts = vec![0i64; monkeys.len()]; // Can't use array here with monkeys.len()?

  for monkey in &mut monkeys {
    monkey.items.reserve(40);
  }
  

  for _ in 0..10000 {
    for i in 0..monkeys.len() {
      let (op, target, items) = {
        let monkey = &mut monkeys[i];
        (
          monkey.op,
          monkey.target,
          monkey.items.drain(..).collect_vec(),
        )
      };
      counts[i] += items.len() as i64;
      for item in items {
        let item = op(item);
        let item = item % modulo;
        let target = target(item);
        monkeys[target].items.push(item)
      }
    }
  }

  let res = counts.into_iter().sorted().rev().take(2).product();
  Ok(res)
}

pub fn run(input: &()) -> AocResult<(i32, i64)> { Ok((part1(input)?, part2(input)?)) }
