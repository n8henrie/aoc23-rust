#![warn(clippy::pedantic)]
use std::result;
use std::str::FromStr;

use aoc::{err, localpath, parse_input, Error, Result};

fn part1(games: &[Game]) -> u32 {
    let limit = Roll {
        red: 12,
        green: 13,
        blue: 14,
    };

    games
        .iter()
        .filter_map(|game| {
            if game.rolls.iter().all(|roll| roll.possible(&limit)) {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

fn part2(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|game| {
            let limit = game.find_limit();
            limit.red * limit.green * limit.blue
        })
        .sum()
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    rolls: Vec<Roll>,
}

impl Game {
    fn find_limit(&self) -> Roll {
        self.rolls.iter().fold(Roll::default(), |acc, roll| Roll {
            red: acc.red.max(roll.red),
            green: acc.green.max(roll.green),
            blue: acc.blue.max(roll.blue),
        })
    }
}

#[derive(Debug, Default, PartialEq, PartialOrd)]
struct Roll {
    blue: u32,
    red: u32,
    green: u32,
}

impl Roll {
    /// A roll is possible if all members of self are <= limit
    fn possible(&self, limit: &Self) -> bool {
        self.red <= limit.red && self.green <= limit.green && self.blue <= limit.blue
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let mut at_colon = s.split(':');
        let id: u32 = at_colon
            .next()
            .ok_or_else(|| err!("no id found in {s}"))
            .map(|pre_colon| pre_colon.split_whitespace())
            .and_then(|mut splitter| {
                if let (Some("Game"), Some(num)) = (splitter.next(), splitter.next()) {
                    Ok(num.parse()?)
                } else {
                    Err(err!("unable to parse game id: {s}"))
                }
            })?;

        let rolls: Vec<Roll> = at_colon
            .next()
            .ok_or_else(|| err!("no rolls in {s}"))
            .and_then(|post_colon| post_colon.split(';').map(str::parse).collect())?;
        Ok(Game { id, rolls })
    }
}

impl FromStr for Roll {
    type Err = Error;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        s.split(',').try_fold(Roll::default(), |acc, chunk| {
            let mut words = chunk.split_whitespace();
            match (words.next().map(str::parse), words.next()) {
                (Some(Ok(green)), Some("green")) => Ok(Roll {
                    green,
                    blue: acc.blue,
                    red: acc.red,
                }),
                (Some(Ok(blue)), Some("blue")) => Ok(Roll {
                    green: acc.green,
                    blue,
                    red: acc.red,
                }),
                (Some(Ok(red)), Some("red")) => Ok(Roll {
                    green: acc.green,
                    blue: acc.blue,
                    red,
                }),
                _ => Err(err!("unable to parse roll: {chunk}")),
            }
        })
    }
}

fn main() -> Result<()> {
    let parsed = parse_input!(localpath!("input.txt"), Game)?;
    println!("day 02 part 1: {}", part1(&parsed));
    println!("day 02 part 2: {}", part2(&parsed));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_parse() {
        let games: Vec<Game> = EXAMPLE_INPUT
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(str::parse)
            .collect::<Result<_>>()
            .unwrap();
        assert_eq!(
            games[0],
            Game {
                id: 1,
                rolls: vec![
                    Roll {
                        blue: 3,
                        red: 4,
                        ..Default::default()
                    },
                    Roll {
                        blue: 6,
                        red: 1,
                        green: 2
                    },
                    Roll {
                        green: 2,
                        ..Default::default()
                    },
                ]
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input!(EXAMPLE_INPUT, Game).unwrap()), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input!(EXAMPLE_INPUT, Game).unwrap()), 2286);
    }

    #[test]
    fn regression() {
        let parsed = parse_input!(localpath!("input.txt"), Game).unwrap();
        assert_eq!(part1(&parsed), 2237);
    }
}
