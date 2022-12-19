use std::collections::{HashSet, VecDeque};

type Pos = (usize, usize, usize);

fn parse(input: &str) -> ([[[bool; 50]; 50]; 50], Vec<Pos>) {
    let mut output = [[[false; 50]; 50]; 50];
    let mut points = Vec::new();
    for line in input.lines() {
        let coords = line.split(",").map(|n| n.parse::<usize>().unwrap() + 2).collect::<Vec<usize>>();

        let p = (coords[0], coords[1], coords[2]);

        output[p.0][p.1][p.2] = true;
        points.push(p);
    }

   (output, points)
}

fn find_surface_area(grid: &[[[bool; 50]; 50]; 50], points: &Vec<Pos>) -> u32 {
    let mut sides = 0;
    for p in points.iter() {
        if !grid[p.0][p.1][p.2 + 1] {
            sides += 1
        }
        if !grid[p.0][p.1][p.2 - 1] {
            sides += 1
        }

        if !grid[p.0 + 1][p.1][p.2] {
            sides += 1
        }
        if !grid[p.0 - 1][p.1][p.2] {
            sides += 1
        }
        
        if !grid[p.0][p.1 + 1][p.2] {
            sides += 1
        }
        if !grid[p.0][p.1 - 1][p.2] {
            sides += 1
        }
    }

    sides
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, points) = parse(input);

    println!("{points:?}");

    Some(find_surface_area(&grid, &points))
}

fn find_exterior_points(grid: &[[[bool; 50]; 50]; 50]) -> HashSet<Pos> {
    let mut queue = VecDeque::new();
    let mut seen: HashSet<Pos> = HashSet::new();
    let mut points = HashSet::new();
    queue.push_back((1,1,1));

    loop {
        let current = match queue.pop_front() {
            None => break,
            Some(p) => p,
        };

        seen.insert(current);

        if !grid[current.0][current.1][current.2] {
            points.insert(current);
        }

        //println!("{current:?}");
        let mut moves = [
            (current.0, current.1, current.2 + 1),
            (current.0, current.1, current.2 - 1),
            (current.0, current.1 + 1, current.2),
            (current.0, current.1 - 1, current.2),
            (current.0 + 1, current.1, current.2),
            (current.0 - 1, current.1, current.2),
        ];


        for m in moves {
            // We don't need to check the edges because the grid is offset
            if m.0 == 0 || m.1 == 0 || m.2 == 0 {
                continue;
            }
            if m.0 == 50 || m.1 == 50 || m.2 == 50 {
                continue;
            }

            if grid[m.0][m.1][m.2] {
                continue;
            }

            if seen.contains(&m) {
                continue;
            }

            seen.insert(m);

            queue.push_back(m);
        }
    }

    points
}

fn mark_interior_points(grid: &mut [[[bool; 50]; 50]; 50], exterior_points: HashSet<Pos>) {
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            for z in 0..grid[y].len() {
                if !exterior_points.contains(&(x, y, z)) {
                    grid[x][y][z] = true;
                }
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut grid, points) = parse(input);

    println!("{points:?}");

    let exterior_points = find_exterior_points(&grid);
    println!("{exterior_points:?}");

    mark_interior_points(&mut grid, exterior_points);

    Some(find_surface_area(&grid, &points))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
