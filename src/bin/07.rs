use std::collections::HashMap;

#[derive(Debug)]
enum DirEntry {
    Dir(HashMap<String, DirEntry>),
    File(u32),
}

impl DirEntry {
    fn parse(input: &str) -> DirEntry {
        let mut cwd: Vec<String> = Vec::new();
        let mut root = HashMap::new();

        for command in input[2..].split("\n$ ") {
            let (command, output) = command.split_once("\n").unwrap_or((command, ""));

            let (verb, args) = command.split_once(" ").unwrap_or((command, ""));

            match verb {
                "cd" => match args {
                    ".." => {
                        cwd.pop();
                    }
                    "/" => {
                        cwd.truncate(0);
                    }
                    _ => {
                        cwd.push(args.to_string());
                    }
                },
                "ls" => {
                    let mut cdir = &mut root;
                    for dir in &cwd {
                        match cdir.get_mut(&dir.to_string()).unwrap() {
                            DirEntry::Dir(children) => cdir = children,
                            DirEntry::File(_) => panic!("Tried to go into a non-dir"),
                        }
                    }
                    for line in output.split("\n") {
                        let (one, two) = line.split_once(" ").unwrap();
                        match one {
                            "dir" => {
                                cdir.entry(two.to_string())
                                    .or_insert(DirEntry::Dir(HashMap::new()));
                            }
                            size => {
                                cdir.insert(
                                    two.to_string(),
                                    DirEntry::File(size.parse::<u32>().unwrap()),
                                );
                            }
                        }
                    }
                }
                _ => panic!("Found verb {verb} {args}"),
            }
        }

        DirEntry::Dir(root)
    }
}

fn calc_sizes(dirs: &DirEntry, sizes: &mut Vec<u32>) -> u32 {
    match dirs {
        DirEntry::File(size) => *size,
        DirEntry::Dir(children) => {
            let size = children
                .iter()
                .map(|(name, child)| {
                    let size = calc_sizes(child, sizes);
                    size
                })
                .sum();

            sizes.push(size);

            size
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let dirs = DirEntry::parse(input);

    let mut sizes = Vec::new();
    calc_sizes(&dirs, &mut sizes);

    Some(sizes.iter().filter(|x| **x <= 100_000).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let dirs = DirEntry::parse(input);

    let mut sizes = Vec::new();
    let root_size = calc_sizes(&dirs, &mut sizes);

    let target: i64 = (root_size as i64) - 40000000;

    let mut min_diff = 1000000000;
    let mut min_seen = 0;
    for size in sizes {
        let diff = (target - size as i64).abs();
        if diff < min_diff && size as i64 > target {
            min_diff = diff;
            min_seen = size
        }
    }

    Some(min_seen)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
