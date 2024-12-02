advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .split_ascii_whitespace()
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .unzip();

    left.sort();
    right.sort();

    let result: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (*l).abs_diff(*r))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right): (Vec<u32>, Vec<u32>) = input
        .split_ascii_whitespace()
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .unzip();

    let mut total = 0;
    for ln in left.iter() {
        let mut count = 0;

        for rn in right.iter() {
            if ln == rn {
                count += 1
            }
        }

        total += ln * count
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
