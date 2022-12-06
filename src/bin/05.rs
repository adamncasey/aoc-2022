fn read_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();
    let mut lines = input.split("\n").collect::<Vec<&str>>();

    let last = lines.pop().unwrap();
    let num = (last.len() / 4) + 1;
    stacks.resize_with(num, Vec::new);
    for row in lines.iter().rev() {
        for (stack, ch) in row.chars().skip(1).step_by(4).enumerate() {
            if ch == ' ' {
                continue;
            }
            stacks[stack].push(ch);
        }
    }

    stacks
}

fn make_output(stacks: &Vec<Vec<char>>) -> String {
    let mut output = String::new();

    for stack in stacks {
        match stack.last() {
            Some(ch) => output.push(*ch),
            None => {},
        }
    }

    output
}

pub fn part_one(input: &str) -> Option<String> {
    let input = input.split_once("\n\n").unwrap();
    let mut stacks = read_stacks(input.0);

    for line in input.1.split("\n") {
        let nums = line.split(" ").skip(1).step_by(2).map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();

        for _ in 0..nums[0] {
            let moving = stacks[nums[1] - 1].pop().unwrap();
            stacks[nums[2] - 1].push(moving);
        }
    }

    println!("{stacks:?}");

    Some(make_output(&stacks))
}

pub fn part_two(input: &str) -> Option<String> {
    let input = input.split_once("\n\n").unwrap();
    let mut stacks = read_stacks(input.0);

    for line in input.1.split("\n") {
        let nums = line.split(" ").skip(1).step_by(2).map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();

        let target_len = stacks[nums[1] - 1].len() - nums[0];
        let moving = stacks[nums[1] - 1].split_off(target_len);
        stacks[nums[2] - 1].extend(&moving);
    }

    println!("{stacks:?}");

    Some(make_output(&stacks))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
