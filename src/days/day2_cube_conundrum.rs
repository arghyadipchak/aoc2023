use super::AdventDay;

pub struct Day2 {
  pub input: String,
}

impl Day2 {}

impl AdventDay for Day2 {
  fn name(&self) -> &'static str {
    "Cube Conundrum"
  }

  fn part1(&self) -> String {
    self
      .input
      .lines()
      .filter_map(|line| {
        let (game, balls) = line.split_once(':')?;

        if balls.split(|c: char| c.is_ascii_punctuation()).any(|ball| {
          let (count, color) = ball[1..].split_once(' ').unwrap();
          count.parse::<u32>().unwrap()
            > match color.chars().next().unwrap() {
              'r' => 12,
              'g' => 13,
              _ => 14,
            }
        }) {
          None
        } else {
          Some(game[5..].parse::<u32>().ok()?)
        }
      })
      .sum::<u32>()
      .to_string()
  }

  fn part2(&self) -> String {
    self
      .input
      .lines()
      .map(|line| {
        let (mut r, mut g, mut b) = (0, 0, 0);

        line
          .split_once(':')
          .unwrap()
          .1
          .split(|c: char| c.is_ascii_punctuation())
          .for_each(|ball| {
            let (count, color) = ball[1..].split_once(' ').unwrap();
            let count: u32 = count.parse().unwrap();
            match color {
              "red" => {
                if count > r {
                  r = count;
                }
              }
              "green" => {
                if count > g {
                  g = count;
                }
              }
              _ => {
                if count > b {
                  b = count;
                }
              }
            }
          });

        r * g * b
      })
      .sum::<u32>()
      .to_string()
  }
}
