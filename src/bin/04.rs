#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn read(input: &str) -> Range {
        let nums: Vec<u32> = input.split("-").map(|idx| idx.parse::<u32>().unwrap()).collect();

        Range {
            start: nums[0],
            end: nums[1],
        }
    }

    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }
    
    fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let input_ranges: Vec<Vec<Range>> = input.split("\n").map(|line| line.split(",").map(Range::read).collect::<Vec<Range>>()).collect();

    println!("{input_ranges:?}");

    Some(input_ranges.iter().filter(|r| r[0].contains(&r[1]) || r[1].contains(&r[0])).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_ranges: Vec<Vec<Range>> = input.split("\n").map(|line| line.split(",").map(Range::read).collect::<Vec<Range>>()).collect();

    println!("{input_ranges:?}");

    Some(input_ranges.iter().filter(|r| r[0].overlaps(&r[1]) || r[1].overlaps(&r[0])).count() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
