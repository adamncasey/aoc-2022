use serde::Deserialize;

use std::cmp::Ordering;

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(untagged)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

impl Packet {
    fn ordered(&self, other: &Packet) -> Ordering {
        match self {
            Packet::Integer(left) => match other {
                Packet::Integer(right) => {
                    if left < right {
                        Ordering::Less
                    } else if left > right {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                }
                Packet::List(_) => Packet::List(vec![self.clone()]).ordered(other),
            },
            Packet::List(left) => match other {
                Packet::Integer(right) => self.ordered(&Packet::List(vec![other.clone()])),
                Packet::List(right) => {
                    let mut right_iter = right.iter();
                    for item in left {
                        match right_iter.next() {
                            None => return Ordering::Greater,
                            Some(right_item) => match item.ordered(right_item) {
                                Ordering::Equal => continue,
                                other => return other,
                            },
                        }
                    }

                    if left.len() < right.len() {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                }
            },
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Packet>> {
    input
        .split("\n\n")
        .map(|pair| {
            pair.split("\n")
                .map(|p| {
                    println!("{p:?}");
                    serde_json::from_str(p).unwrap()
                })
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = parse(input);

    println!("{pairs:?}");

    let mut total = 0;

    for (idx, pair) in pairs.iter().enumerate() {
        match pair[0].ordered(&pair[1]) {
            Ordering::Equal | Ordering::Less => total += idx as u32 + 1,
            _ => {}
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut packets: Vec<Packet> = Vec::new();

    let key_2 = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    let key_6 = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);

    packets.push(key_2.clone());
    packets.push(key_6.clone());

    input.split("\n\n").for_each(|pair| {
        pair.split("\n").for_each(|p| {
            packets.push(serde_json::from_str(p).unwrap());
        })
    });

    println!("{packets:?}");

    packets.sort_by(|l, r| l.ordered(r));

    println!("{packets:?}");

    let mut key = 1;

    for (idx, p) in packets.iter().enumerate() {
        if p == &key_2 || p == &key_6 {
            key *= idx as u32 + 1;
        }
    }

    Some(key)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);

    // not 4236 (too low)
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
