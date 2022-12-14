use std::collections::{HashSet, VecDeque};

type Pos = (usize, usize, u8);

#[derive(Debug)]
struct Problem {
    map: Vec<Vec<u8>>,
    end: Pos,
    start: Pos,
}

fn parse(input: &str) -> Problem {
    let mut start = None;
    let mut end = None;
    let mut map = Vec::new();
    for (row, lines) in input.lines().enumerate() {
        let mut line = Vec::new();

        for (col, ch) in lines.chars().enumerate() {
            match ch {
                'S' => {
                    start = Some((row, col, 0));
                    line.push(0);
                }
                'E' => {
                    end = Some((row, col, 25));
                    line.push(25);
                }
                'a'..='z' => line.push((ch as u32 - 'a' as u32) as u8),
                _ => panic!("Unexpected {ch}"),
            }
        }

        map.push(line);
    }

    Problem {
        map,
        end: end.unwrap(),
        start: start.unwrap(),
    }
}

fn is_safe_move(pos: Pos, map: &Vec<Vec<u8>>, old_pos: Pos) -> bool {
    let height = map[pos.0][pos.1];

    let height_diff = (pos.2 as i16) - (old_pos.2 as i16);

    height == pos.2 && height_diff <= 1
}

fn raw_moves(pos: Pos, next_height: u8, xlen: usize, ylen: usize, moves: &mut Vec<Pos>) {
    if pos.0 > 0 {
        moves.push((pos.0 - 1, pos.1, next_height));
    }
    if pos.0 < xlen - 1 {
        moves.push((pos.0 + 1, pos.1, next_height));
    }
    if pos.1 > 0 {
        moves.push((pos.0, pos.1 - 1, next_height));
    }
    if pos.1 < ylen - 1 {
        moves.push((pos.0, pos.1 + 1, next_height));
    }
}

fn available_moves(
    move_num: u32,
    mut prev_moves: Vec<Pos>,
    pos: (usize, usize, u8),
    map: &Vec<Vec<u8>>,
) -> Vec<(u32, Pos, Vec<Pos>)> {
    let mut moves: Vec<Pos> = Vec::new();

    prev_moves.push(pos);

    let xlen = map.len();
    let ylen = map[0].len();

    raw_moves(pos, pos.2, xlen, ylen, &mut moves);

    for h in 0..=pos.2 {
        raw_moves(pos, h, xlen, ylen, &mut moves);
    }

    if pos.2 < 25 {
        raw_moves(pos, pos.2 + 1, xlen, ylen, &mut moves);
    }

    moves
        .into_iter()
        .filter(|m| is_safe_move(*m, map, pos))
        .map(|m| (move_num, m, prev_moves.clone()))
        .collect()
}

fn find_end(p: &Problem) -> u32 {
    let mut move_queue: VecDeque<(u32, Pos, Vec<Pos>)> = VecDeque::from(available_moves(
        1,
        Vec::new(),
        (p.start.0, p.start.1, 0),
        &p.map,
    ));
    let mut seen = HashSet::new();

    loop {
        let next_move = match move_queue.pop_front() {
            None => panic!("Found no route to dest"),
            Some(m) => m,
        };

        if seen.contains(&next_move.1) {
            continue;
        }

        if next_move.1 .0 == p.end.0 && next_move.1 .1 == p.end.1 && next_move.1 .2 == 25 {
            return next_move.0;
        } else {
            let new_moves =
                available_moves(next_move.0 + 1, next_move.2.clone(), next_move.1, &p.map);
            move_queue.extend(new_moves);
        }

        seen.insert(next_move.1);
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut p = parse(input);

    println!("{p:?}");

    let mut min_steps = 1000;

    for (row, heights) in p.map.iter().enumerate() {
        for (col, height) in heights.iter().enumerate() {
            if col == 0 {
                p.start = (row, col, 0);

                min_steps = std::cmp::min(find_end(&p), min_steps);
            }
        }
    }

    Some(min_steps)
}

pub fn part_one(input: &str) -> Option<u32> {
    let p = parse(input);

    println!("{p:?}");
    Some(find_end(&p))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
