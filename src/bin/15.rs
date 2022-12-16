use std::collections::HashSet;

type Pos = (i32, i32);
type PosDist = (Pos, i32);

fn parse(input: &str) -> (Vec<PosDist>, HashSet<Pos>) {
    // Sensor at x=2, y=18: closest beacon is at x=-2, y=15

    let mut output = Vec::new();
    let mut beacons = HashSet::new();

    for line in input.lines() {
        let (_, rest) = line.split_once("Sensor at x=").unwrap();

        let (sensor, beacon) = rest.split_once(": closest beacon is at x=").unwrap();

        let (s_x, s_y) = sensor.split_once(", y=").unwrap();
        let (s_x, s_y) = (s_x.parse::<i32>().unwrap(), s_y.parse::<i32>().unwrap());
        let (b_x, b_y) = beacon.split_once(", y=").unwrap();
        let (b_x, b_y) = (b_x.parse::<i32>().unwrap(), b_y.parse::<i32>().unwrap());

        beacons.insert((b_x, b_y));

        output.push(((s_x, s_y), (s_x - b_x).abs() + (s_y - b_y).abs()));
    }

    (output, beacons)
}

pub fn part_one(input: &str, row: i32) -> Option<u32> {
    let (data, beacons) = parse(input);

    let mut counter = 0;

    for x in -10000000..=10000000 {
        if beacons.contains(&(x, row)) {
            // This square can contain a beacon because it does.
            continue;
        }

        let (seen, _) = sensor_sees(x, row, &data);

        if seen {
            counter += 1;
        }
    }

    Some(counter)
}

pub fn part_one_solve(input: &str) -> Option<u32> {
    part_one(input, 2000000)
}

fn sensor_sees(x: i32, y: i32, sensors: &Vec<PosDist>) -> (bool, i32) {
    for sensor in sensors.iter() {
        let dist = (x - sensor.0.0).abs() + (y - sensor.0.1).abs();
    
        if dist <= sensor.1 {
            return (true, sensor.1 - dist + 1);
        }
    }

    return (false, 1);
}

pub fn part_two(input: &str, max_coord: i32) -> Option<i64> {
    let (data, _) = parse(input);

    println!("{data:?}");

    for x in 0..=max_coord {
        let mut y = 0;

        loop {
            if y > max_coord {
                break;
            }

            let (seen, remaining_dist) = sensor_sees(x, y, &data);

            if !seen {
                println!("Found {x}, {y}");
                return Some((x as i64 * 4000000) + y as i64);
            }


            y += remaining_dist;
        }
    }

    None
}

fn part_two_solve(input: &str) -> Option<i64> {
    part_two(input, 4000000)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one_solve, input);
    advent_of_code::solve!(2, part_two_solve, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input, 10), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input, 20), Some(56000011));
    }
}
