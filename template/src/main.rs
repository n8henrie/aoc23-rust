// #![warn(clippy::pedantic)]
use aoc::{err, localpath, parse_input, Error, Result};

const INPUT: &str = include_str!("../input.txt");

fn part1() -> Result<u32> {
    todo!()
}

fn part2() -> Result<u32> {
    todo!()
}

fn main() -> Result<()> {
    // let input = parse_input!(localpath!("input.txt"))?;
    // println!("day part 1: {}", part1(&input)?);
    // println!("day part 2: {}", part2(&input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = r#"
"#;

    #[test]
    fn test_part1() {
        assert!(false);
    }

    #[test]
    fn test_part2() {
        assert!(false);
    }

    #[test]
    /// Once a solution is found, enter it here to ensure that further
    /// optimizations don't break the code.
    fn regression() {
        assert!(false);
        // assert_eq!(part1(INPUT).unwrap(), _);
        // assert_eq!(part2(INPUT).unwrap(), _);
    }
}
