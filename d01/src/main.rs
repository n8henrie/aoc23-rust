// #![warn(clippy::pedantic)]
use aoc::{err, Result};

const INPUT: &str = include_str!("../input.txt");

fn find_map_inc_strings(line: &str, reverse: bool) -> Option<u32> {
    let words = vec![
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let indices: Box<dyn Iterator<Item = _>> = if reverse {
        Box::new(line.char_indices().rev())
    } else {
        Box::new(line.char_indices())
    };
    indices.into_iter().find_map(|(idx, char)| {
        char.to_digit(10).or_else(|| {
            words.iter().find_map(|(w, val)| {
                if line.get(idx..)?.starts_with(w) {
                    Some(*val)
                } else {
                    None
                }
            })
        })
    })
}

fn line_score(line: impl AsRef<str>, include_strings: bool) -> Result<u32> {
    let line = line.as_ref();

    let (first, last) = if include_strings {
        (
            find_map_inc_strings(line, false),
            find_map_inc_strings(line, true),
        )
    } else {
        let to_digit = |c| char::to_digit(c, 10);
        (
            line.chars().find_map(to_digit),
            line.chars().rev().find_map(to_digit),
        )
    };

    if let (Some(first), Some(last)) = (first, last) {
        Ok(first * 10 + last)
    } else {
        Err(err!("No digit found in {line}"))
    }
}

fn solve(input: &str, include_strings: bool) -> Result<u32> {
    input.lines().try_fold(0, |acc, line| {
        let line = line.trim();
        Ok(if line.is_empty() {
            acc
        } else {
            acc + line_score(line, include_strings)?
        })
    })
}

fn part1(input: &str) -> Result<u32> {
    solve(input, false)
}

fn part2(input: &str) -> Result<u32> {
    solve(input, true)
}

fn main() -> Result<()> {
    println!("day 01 part 1: {}", solve(INPUT, false)?);
    println!("day 01 part 2: {}", solve(INPUT, true)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static PART1_EXAMPLE_INPUT: &str = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;

    static PART2_EXAMPLE_INPUT: &str = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

    #[test]
    fn test_part1() {
        assert!(PART1_EXAMPLE_INPUT
            .lines()
            .filter(|l| !l.is_empty())
            .map(|line| line_score(line, false))
            .zip([12, 38, 15, 77])
            .inspect(|v| {
                dbg!(v);
            })
            .all(|(left, right)| left.unwrap() == right));
        assert_eq!(part1(PART1_EXAMPLE_INPUT).unwrap(), 142);
    }

    #[test]
    fn test_part2() {
        assert!(PART2_EXAMPLE_INPUT
            .lines()
            .filter(|l| !l.is_empty())
            .inspect(|v| {
                dbg!(v);
            })
            .map(|line| line_score(line, true))
            .zip([29, 83, 13, 24, 42, 14, 76])
            .inspect(|v| {
                dbg!(v);
            })
            .all(|(left, right)| left.unwrap() == right));
        assert_eq!(part2(PART2_EXAMPLE_INPUT).unwrap(), 281);
    }

    #[test]
    fn regression() {
        assert_eq!(part1(INPUT).unwrap(), 53334);
        assert_eq!(part2(INPUT).unwrap(), 52834);
    }
}
