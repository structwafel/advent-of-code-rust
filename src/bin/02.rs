advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let groups = create_groups(input);

    let mut count = 0;

    for group in groups {
        if check_group_correct(&group) {
            count += 1
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let groups = create_groups(input);

    let mut count = 0;

    'outer: for group in groups {
        if check_group_correct(&group) {
            count += 1;
            continue;
        }

        for i in 0..group.len() {
            let mut group_clone = group.clone();
            group_clone.remove(i);
            if check_group_correct(&group_clone) {
                count += 1;
                continue 'outer;
            }
        }
    }

    Some(count)
}

fn check_group_correct(group: &[u32]) -> bool {
    group.len() > 0 && is_max_diff(&group) && is_ordered(&group)
}

fn is_max_diff(group: &[u32]) -> bool {
    for window in group.windows(2) {
        if let [x, y] = window {
            let diff = x.abs_diff(*y);

            if diff < 1 || diff > 3 {
                return false;
            }
        }
    }
    return true;
}

fn is_ordered(group: &[u32]) -> bool {
    group.windows(2).all(|x| x[0] < x[1]) || group.windows(2).all(|x| x[0] > x[1])
}

fn create_groups(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n")
        .into_iter()
        .collect::<Vec<&str>>()
        .iter()
        .map(|report| {
            report
                .split_ascii_whitespace()
                .into_iter()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
