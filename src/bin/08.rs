use std::collections::HashSet;

fn read_file(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|t| t.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect()
}

fn is_visible(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    if x == 0 || y == 0 {
        return true;
    }

    if x == (trees.len() - 1) || y == (trees.len() - 1) {
        return true;
    }

    let height = trees[x][y];

    if trees[x][0..y].iter().max().unwrap() < &height {
        return true;
    }

    if trees[x][(y + 1)..].iter().max().unwrap() < &height {
        return true;
    }

    if trees[0..x].iter().map(|r| r[y]).max().unwrap() < height {
        return true;
    }

    if trees[(x+1)..].iter().map(|r| r[y]).max().unwrap() < height {
        return true;
    }

    return false;
}

fn scenic_score(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let height = trees[x][y];

    if trees[x][0..y].iter().max().unwrap() < &height {
        return true;
    }

    if trees[x][(y + 1)..].iter().max().unwrap() < &height {
        return true;
    }

    if trees[0..x].iter().map(|r| r[y]).max().unwrap() < height {
        return true;
    }

    if trees[(x+1)..].iter().map(|r| r[y]).max().unwrap() < height {
        return true;
    }

    return false;
}

pub fn part_one(input: &str) -> Option<u32> {
    let trees = read_file(input);

    let mut count = 0;
    for x in 0..trees.len() {
        for y in 0..trees.len() {
            if is_visible(&trees, x, y) {
                count += 1;
            }
        }
    }

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
