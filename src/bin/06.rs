use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    for (idx, window) in input.as_bytes().windows(4).enumerate() {
        let [a, b, c, d]: [u8; 4] = window.try_into().unwrap();
        if a != b && a != c && a != d && b != c && b != d && c != d {
            return Some(idx as u32 + 4);
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    for (idx, window) in input.as_bytes().windows(14).enumerate() {
        let mut set: HashSet<u8> = HashSet::new();
        for b in window {
            set.insert(*b);
        }

        if set.len() == 14 {
            return Some(idx as u32 + 14);
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(6));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(23));
    }
}
