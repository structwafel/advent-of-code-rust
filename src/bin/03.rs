advent_of_code::solution!(3);
use std::sync::LazyLock;

use regex::Regex;

static MUL_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap());
static NUM_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d{1,3}").unwrap());
static PART_TWO_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\))").unwrap());

pub fn part_one(input: &str) -> Option<u32> {
    let total: u32 = MUL_RE
        .find_iter(input)
        .into_iter()
        .map(|m| multiply_from_mul(m.as_str()))
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let matches: Vec<&str> = PART_TWO_RE
        .find_iter(input)
        .into_iter()
        .map(|m| m.as_str())
        .collect();

    let mut enabled = true;
    let mut total = 0;
    for m in matches {
        match m {
            _ if m.starts_with("m") && enabled => total += multiply_from_mul(m),
            _ if m.starts_with("do()") => enabled = true,
            _ => enabled = false,
        }
    }

    Some(total)
}

fn multiply_from_mul(mul_str: &str) -> u32 {
    let mut numbers = NUM_RE.find_iter(mul_str);
    let (l, r): (u32, u32) = (
        numbers.next().unwrap().as_str().parse().unwrap(),
        numbers.next().unwrap().as_str().parse().unwrap(),
    );
    l * r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
