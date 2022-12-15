use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Square {
    Air,
    Rock,
    Sand,
}

impl Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Square::Air => '.',
                Square::Rock => '#',
                Square::Sand => 'O',
            }
        )
    }
}

#[derive(Debug)]
struct Cave {
    map: Vec<Vec<Square>>,
    max_y: usize,
    min_x: usize,
}

impl Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.map.iter() {
            for sq in line.iter() {
                write!(f, "{sq}");
            }
            writeln!(f, "");
        }

        write!(f, "")
    }
}

fn parse(input: &str, insert_floor: bool) -> Cave {
    let mut max_x = 0;
    let mut min_x = 100000;
    let mut max_y = 0;
    let mut min_y = 100000;

    let mut rock_strings = input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|pair| {
                    let (x, y) = pair.split_once(",").unwrap();
                    let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());

                    max_x = std::cmp::max(x, max_x);
                    max_y = std::cmp::max(y, max_y);
                    min_y = std::cmp::min(y, min_y);
                    min_x = std::cmp::min(x, min_x);

                    (x, y)
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<Vec<(usize, usize)>>>();

    if insert_floor {
        rock_strings.push(vec![(250, max_y + 2), (750, max_y + 2)]);
        min_x = 250;
        max_x = 750;
        max_y = max_y + 2;
    }

    // create map
    let mut map = Vec::new();

    for row in 0..=max_y {
        map.push(vec![Square::Air; max_x - min_x + 1]);
    }

    for rock_string in rock_strings {
        for rock_pair in rock_string.windows(2) {
            if let &[p1, p2] = rock_pair {
                println!("Drawing {p1:?} {p2:?}");
                if p1.0 != p2.0 {
                    for col in p1.0..=p2.0 {
                        map[p1.1][col - min_x] = Square::Rock;
                    }
                    for col in p2.0..=p1.0 {
                        map[p1.1][col - min_x] = Square::Rock;
                    }
                }
                if p1.1 != p2.1 {
                    for row in p1.1..=p2.1 {
                        map[row][p1.0 - min_x] = Square::Rock;
                    }
                    for row in p2.1..=p1.1 {
                        map[row][p1.0 - min_x] = Square::Rock;
                    }
                }
            }
        }
    }

    // for every rock string, place rocks.

    Cave { map, max_y, min_x }
}

fn simulate_cave(cave: &mut Cave) -> u32 {
    let mut counter = 0;

    loop {
        // insert sand
        let mut s: (i32, i32) = (500 - cave.min_x as i32, 0);

        if cave.map[s.1 as usize][s.0 as usize] == Square::Sand {
            println!("Cave is full");
            return counter;
        }

        //println!("{counter} {cave}");

        loop {
            // get sand move
            let possible_moves = [(s.0, s.1 + 1), (s.0 - 1, s.1 + 1), (s.0 + 1, s.1 + 1)];

            let mut moved = false;
            for m in possible_moves {
                //println!("{s:?} {m:?}");
                if m.1 > cave.max_y as i32 || m.0 < 0 || m.0 >= cave.map[0].len() as i32 {
                    println!("Found sand falling into abyss {m:?}");
                    return counter;
                }

                if cave.map[m.1 as usize][m.0 as usize] == Square::Air {
                    cave.map[s.1 as usize][s.0 as usize] = Square::Air;
                    s = m;
                    cave.map[s.1 as usize][s.0 as usize] = Square::Sand;
                    moved = true;
                    break;
                }
            }

            if !moved {
                cave.map[s.1 as usize][s.0 as usize] = Square::Sand;
                counter += 1;
                break;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cave = parse(input, false);

    println!("{cave:?}");

    Some(simulate_cave(&mut cave))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cave = parse(input, true);

    println!("{cave:?}");

    Some(simulate_cave(&mut cave))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
