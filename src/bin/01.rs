pub fn part_two(input: &str) -> Option<u32> {
    let mut sorted = input
        .split("\n\n")
        .map(|e| {
            e.split("\n")
                .map(|num| num.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    sorted.sort();
    let max = sorted.iter().rev().take(3).sum::<u32>();

    Some(max)
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .map(|e| {
            e.split("\n")
                .map(|num| num.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
}
