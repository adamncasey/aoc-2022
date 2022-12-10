use std::cmp::{max, min};
use std::collections::HashSet;

fn adjust_h_pos(instr: &str, pos: (i32, i32)) -> (i32, i32) {
    match instr {
        "U" => (pos.0, pos.1 + 1),
        "D" => (pos.0, pos.1 - 1),
        "L" => (pos.0 - 1, pos.1),
        "R" => (pos.0 + 1, pos.1),
        _ => panic!("Bad input {instr}"),
    }
}

fn adjust_t_pos(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let diff = (head.0 - tail.0, head.1 - tail.1);

    if diff.0.abs() > 1 {
        if diff.1 == 0 {
            if diff.0 < 0 {
                (tail.0 - 1, tail.1)
            } else {
                (tail.0 + 1, tail.1)
            }
        } else {
            if diff.0 < 0 {
                (tail.0 - 1, tail.1 + max(min(diff.1, 1), -1))
            } else {
                (tail.0 + 1, tail.1 + max(min(diff.1, 1), -1))
            }
        }
    } else if diff.1.abs() > 1 {
        if diff.0 == 0 {
            if diff.1 < 0 {
                (tail.0, tail.1 - 1)
            } else {
                (tail.0, tail.1 + 1)
            }
        } else {
            if diff.1 < 0 {
                (tail.0 + max(min(diff.0, 1), -1), tail.1 - 1)
            } else {
                (tail.0 + max(min(diff.0, 1), -1), tail.1 + 1)
            }
        }
    } else {
        // we don't need to fix tail
        tail
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut h_pos: (i32, i32) = (0, 0);
    let mut t_pos: (i32, i32) = (0, 0);

    let mut t_seen = HashSet::new();

    for line in input.lines() {
        let (instr, num) = line.split_once(" ").unwrap();
        let num = num.parse::<u32>().unwrap();

        for step in 0..num {
            h_pos = adjust_h_pos(instr, h_pos);
            t_pos = adjust_t_pos(h_pos, t_pos);

            if (h_pos.0 - t_pos.0).abs() > 2 || (h_pos.1 - t_pos.1).abs() > 2 {
                println!("Went wrong at {instr} {step} {h_pos:?} {t_pos:?}");
                panic!();
            }

            t_seen.insert(t_pos);

            println!("{h_pos:?} {t_pos:?}");
        }
    }

    Some(t_seen.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rope: [(i32, i32); 10] = [(0, 0); 10];

    let mut t_seen = HashSet::new();

    for line in input.lines() {
        let (instr, num) = line.split_once(" ").unwrap();
        let num = num.parse::<u32>().unwrap();

        for step in 0..num {
            rope[0] = adjust_h_pos(instr, rope[0]);

            let mut prev = rope[0];
            for mut knot in rope[1..].iter_mut() {
                println!("{prev:?} {knot:?}");
                let new_pos = adjust_t_pos(prev, *knot);
                knot.0 = new_pos.0;
                knot.1 = new_pos.1;

                if (prev.0 - knot.0).abs() > 2 || (prev.1 - knot.1).abs() > 2 {
                    panic!("Went wrong at {instr} {step} {prev:?} {knot:?}");
                }

                prev = *knot;
            }

            t_seen.insert(rope[9]);

            println!("{rope:?}");
        }
    }

    Some(t_seen.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
