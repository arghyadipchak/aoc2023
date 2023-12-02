mod day1_trebuchet;
mod day2_cube_conundrum;

use day1_trebuchet::Day1;
use day2_cube_conundrum::Day2;

pub trait AdventDay {
  fn name(&self) -> &'static str {
    ""
  }

  fn part1(&self) -> String {
    String::new()
  }

  fn part2(&self) -> String {
    String::new()
  }
}

pub fn get_day(n: u8, input: String) -> Box<dyn AdventDay> {
  match n {
    1 => Box::new(Day1 { input }),
    2 => Box::new(Day2 { input }),
    _ => todo!(),
  }
}
