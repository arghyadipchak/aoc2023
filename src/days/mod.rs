mod day1_trebuchet;

use day1_trebuchet::Day1;

pub trait AdventDay {
  const NAME: &'static str;

  fn name(&self) -> &'static str {
    Self::NAME
  }

  fn part1(&self) -> String {
    String::new()
  }

  fn part2(&self) -> String {
    String::new()
  }
}

pub fn get_day(n: u8, input: String) -> impl AdventDay {
  match n {
    1 => Day1 { input },
    _ => todo!(),
  }
}
