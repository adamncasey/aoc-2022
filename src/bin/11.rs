use std::collections::HashMap;

#[derive(Debug)]
enum Operation {
    Add(u64),
    Mult(u64),
    Square,
}

impl Operation {
    fn parse(input: &str) -> Operation {
        match input.split_once(" ") {
            Some(("*", "old")) => Operation::Square,
            Some(("*", num)) => Operation::Mult(num.parse::<u64>().unwrap()),
            Some(("+", num)) => Operation::Add(num.parse::<u64>().unwrap()),
            _ => panic!("Bad op parse {input}"),
        }
    }

    fn apply(&self, operand: u64) -> u64 {
        match self {
            Operation::Add(val) => operand + val,
            Operation::Mult(val) => operand * val,
            Operation::Square => operand * operand,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    op: Operation,
    test: u64,
    true_dest: usize,
    false_dest: usize,
}

impl Monkey {
    fn parse(input: &str) -> Monkey {
        let lines = input.lines().collect::<Vec<&str>>();

        let items = lines[1]
            .split_once("  Starting items: ")
            .unwrap()
            .1
            .split(", ")
            .map(|i| i.parse::<u64>().unwrap())
            .collect();

        let op = Operation::parse(lines[2].split_once("  Operation: new = old ").unwrap().1);

        let test = lines[3]
            .split_once("  Test: divisible by ")
            .unwrap()
            .1
            .parse::<u64>()
            .unwrap();

        let true_dest = lines[4]
            .split_once("    If true: throw to monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        let false_dest = lines[5]
            .split_once("    If false: throw to monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();

        Monkey {
            items,
            op,
            test,
            true_dest,
            false_dest,
        }
    }
}

fn monkey_business(monkeys: &mut [Monkey], rounds: usize, reducer: u64) -> u64 {
    let mut monkey_see: HashMap<usize, u64> = HashMap::new();

    let common_multiple = monkeys.iter().map(|m| m.test).fold(1, |acc, x| acc * x);
    println!("Common multiple {common_multiple}");

    for round in 0..rounds {
        for m in 0..monkeys.len() {
            let mut thrown: Vec<(usize, u64)> = Vec::new();
            let monkey: &Monkey = &monkeys[m];

            for item in &monkey.items {
                *monkey_see.entry(m).or_default() += 1;
                // new worry level calc
                let worry = ((monkey.op.apply(*item)) / reducer) % common_multiple;

                let dest = if worry % monkey.test == 0 {
                    monkey.true_dest
                } else {
                    monkey.false_dest
                };

                thrown.push((dest, worry));
            }
            monkeys[m].items = Vec::new();

            for throw in thrown {
                monkeys[throw.0].items.push(throw.1);
            }
        }
    }

    let mut inspections = monkey_see.values().cloned().collect::<Vec<u64>>();
    inspections.sort();

    inspections.pop().unwrap() * inspections.pop().unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::parse).collect();

    println!("{monkeys:?}");
    Some(monkey_business(&mut monkeys, 20, 3))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::parse).collect();

    println!("{monkeys:?}");
    Some(monkey_business(&mut monkeys, 10000, 1))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
