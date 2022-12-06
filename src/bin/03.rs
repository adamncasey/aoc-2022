use std::collections::{HashMap, HashSet};

fn most_shared(bag1: &str, bag2: &str, bag3: &str) -> char {
    let mut items: HashMap<char, u32> = HashMap::new();

    for bag in [bag1, bag2, bag3] {
        let mut seen = HashSet::new();
        for ch in bag.chars() {
            seen.insert(ch);
        }

        for ch in seen {
            *items.entry(ch).or_insert(0) += 1;
        }
    }

    let mut max_seen = 0;
    let mut max_ch = 'a';
    for (ch, count) in items {
        if count > max_seen {
            max_seen = count;
            max_ch = ch;
        }
    }

    if max_seen == 0 {
        panic!("We were told we wouldn't get here :(");
    }

    max_ch
}

fn common_item(bag: &str) -> char {
    let bag: (&str, &str) = bag.split_at(bag.len() / 2);

    most_shared(bag.0, bag.1, "")
}

fn score_item(ch: char) -> u32 {
    match ch {
        'a'..='z' => (ch as u8 - 'a' as u8 + 1).into(),
        'A'..='Z' => (ch as u8 - 'A' as u8 + 27).into(),
        _ => panic!("Unexpected {ch}"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.split("\n").map(common_item).map(score_item).sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.split("\n").collect::<Vec<&str>>().chunks_exact(3).map(|bags| most_shared(bags[0], bags[1], bags[2])).map(score_item).sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
