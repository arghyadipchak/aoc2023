mod days;

use std::fs;

use days::AdventDay;

fn read_input(n: u8) -> String {
  fs::read_to_string(format!("input/day{n}.txt")).unwrap_or_default()
}

fn main() {
  for n in 1..=1 {
    let day = days::get_day(n, read_input(n));

    println!("--- Day {n}: {} ---", day.name());
    println!("  Part 1: {}", day.part1());
    println!("  Part 2: {}", day.part2());
  }
}
