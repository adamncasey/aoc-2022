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

    if trees[(x + 1)..].iter().map(|r| r[y]).max().unwrap() < height {
        return true;
    }

    return false;
}

fn scenic_score(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> u32 {
    let height = trees[x][y];
    let mut total_score = 1;

    let mut last_height = 0;
    let mut l_score = 0;
    for col in (0..y).rev() {
        let t = trees[x][col];
        // So it turns out you CAN see occluded trees?!
        //if t >= last_height {
        l_score += 1;
        last_height = t;
        //}

        if t >= height {
            break;
        }
    }
    total_score *= l_score;

    let mut last_height = 0;
    let mut r_score = 0;
    for col in (y + 1)..(trees[x].len()) {
        let t = trees[x][col];
        //if t >= last_height {
        r_score += 1;
        last_height = t;
        //}

        if t >= height {
            break;
        }
    }
    total_score *= r_score;

    let mut last_height = 0;
    let mut u_score = 0;
    for row in (0..x).rev() {
        let t = trees[row][y];
        //if t >= last_height {
        u_score += 1;
        last_height = t;
        //}

        if t >= height {
            break;
        }
    }
    total_score *= u_score;

    let mut last_height = 0;
    let mut d_score = 0;
    for row in (x + 1)..(trees.len()) {
        let t = trees[row][y];
        //if t >= last_height {
        d_score += 1;
        last_height = t;
        //}

        if t >= height {
            break;
        }
    }
    total_score *= d_score;

    println!("Scenic {x},{y} {total_score}: {u_score} {l_score} {d_score} {r_score}");

    total_score
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
    let trees = read_file(input);

    trees
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(|(y, t)| scenic_score(&trees, x, y))
                .max()
                .unwrap()
        })
        .max()
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
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
