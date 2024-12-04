advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .into_iter()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut counter = 0;

    let grid_refs: Vec<&[char]> = grid.iter().map(|row| row.as_slice()).collect();

    for (x, row) in grid.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if *c == 'X' {
                counter += xmas_count_from_here(x, y, &grid_refs)
            }
        }
    }

    Some(counter)
}

fn xmas_count_from_here(x: usize, y: usize, grid: &[&[char]]) -> u32 {
    let directions: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let xmas = ['X', 'M', 'A', 'S'];

    directions
        .iter()
        .filter(|&&(dx, dy)| {
            (0..4).all(|i| {
                // beehhhhh ugly but it works
                let new_x = if dx >= 0 {
                    x.checked_add((dx * i) as usize)
                } else {
                    x.checked_sub((-dx * i) as usize)
                };
                let new_y = if dy >= 0 {
                    y.checked_add((dy * i) as usize)
                } else {
                    y.checked_sub((-dy * i) as usize)
                };

                matches!(
                    (new_x, new_y),
                    (Some(nx), Some(ny)) if get_coord(nx, ny, grid) == Some(xmas[i as usize])
                )
            })
        })
        .count() as u32
}

fn get_coord(x: usize, y: usize, grid: &[&[char]]) -> Option<char> {
    grid.get(x)?.get(y).cloned()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
