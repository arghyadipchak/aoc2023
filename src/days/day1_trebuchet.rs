use lazy_static::lazy_static;
use regex::Regex;

use super::AdventDay;

lazy_static! {
  static ref DIGIT_REGEX: Regex =
    Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
  static ref DIGIT_REV_REGEX: Regex =
    Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
}

pub struct Day1 {
  pub input: String,
}

impl AdventDay for Day1 {
  const NAME: &'static str = "Trebuchet?!";

  fn part1(&self) -> String {
    self
      .input
      .lines()
      .map(|line| {
        line.chars().find_map(|c| c.to_digit(10)).unwrap() * 10
          + line.chars().rev().find_map(|c| c.to_digit(10)).unwrap()
      })
      .sum::<u32>()
      .to_string()
  }

  fn part2(&self) -> String {
    self
      .input
      .lines()
      .map(|line| {
        (match DIGIT_REGEX.find(line).unwrap().as_str() {
          "1" | "one" => 1,
          "2" | "two" => 2,
          "3" | "three" => 3,
          "4" | "four" => 4,
          "5" | "five" => 5,
          "6" | "six" => 6,
          "7" | "seven" => 7,
          "8" | "eight" => 8,
          "9" | "nine" => 9,
          _ => 0,
        }) * 10
          + match DIGIT_REV_REGEX
            .find(&line.chars().rev().collect::<String>())
            .unwrap()
            .as_str()
          {
            "1" | "eno" => 1,
            "2" | "owt" => 2,
            "3" | "eerht" => 3,
            "4" | "ruof" => 4,
            "5" | "evif" => 5,
            "6" | "xis" => 6,
            "7" | "neves" => 7,
            "8" | "thgie" => 8,
            "9" | "enin" => 9,
            _ => 0,
          }
      })
      .sum::<u32>()
      .to_string()
  }
}
