use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Node {
    flow_rate: u32,
    connected: Vec<(String, u32)>,
}

fn parse(input: &str) -> HashMap<String, Node> {
    // Valve JI has flow rate=21; tunnels lead to valves WI, XG

    let mut output = HashMap::new();

    for line in input.lines() {
        println!("{line}");
        let (_, rest) = line.split_once("Valve ").unwrap();

        let (node, rest) = rest.split_once(" has flow rate=").unwrap();

        let (rate, rest) = rest
            .split_once("; tunnels lead to valves ")
            .unwrap_or_else(|| rest.split_once("; tunnel leads to valve ").unwrap());

        let connected = rest
            .split(", ")
            .map(|n| (String::from(n), 1))
            .collect::<Vec<(String, u32)>>();

        output.insert(
            String::from(node),
            Node {
                flow_rate: rate.parse::<u32>().unwrap(),
                connected,
            },
        );
    }

    output
}

fn cave_dfs(
    caves: &HashMap<String, Node>,
    h_current: &str,
    e_current: &str,
    open: HashSet<String>,
    score: u32,
    flow_rate: u32,
    time_remaining: u32,
    best_so_far: &mut u32,
    max_score_estimates: &HashMap<u32, u32>,
    path: &mut Vec<String>,
    h_free_at: u32,
    e_free_at: u32,
) {
    if time_remaining == 0 {
        if score > *best_so_far {
            *best_so_far = score;
            println!("{h_current} {e_current} {score} {flow_rate} {open:?} {path:?}");
        }
        return;
    }

    let score = if time_remaining % 2 == 0 {
        score + flow_rate
    } else {
        score
    };

    if score
        + 200
        + max_score_estimates
            .get(&(30 - (time_remaining / 2)))
            .unwrap()
        <= *best_so_far
    {
        // Pointless avenue
        return;
    }

    let mut moved = false;

    if h_free_at >= time_remaining && !open.contains(h_current) && time_remaining > 1 {
        let mut new_open = open.clone();
        new_open.insert(h_current.to_string());
        let cave = caves.get(h_current).unwrap();

        //path.push(format!("{time_remaining} h open {h_current}"));
        cave_dfs(
            caves,
            h_current,
            e_current,
            new_open,
            score,
            flow_rate + cave.flow_rate,
            time_remaining - 1,
            best_so_far,
            max_score_estimates,
            path,
            time_remaining - 2,
            e_free_at,
        );

        moved = true;

        //path.pop();
    } else if e_free_at >= time_remaining && !open.contains(e_current) && time_remaining > 1 {
        let mut new_open = open.clone();
        new_open.insert(e_current.to_string());
        let cave = caves.get(e_current).unwrap();
        //path.push(format!("{time_remaining} e open {e_current}"));
        cave_dfs(
            caves,
            h_current,
            e_current,
            new_open,
            score,
            flow_rate + cave.flow_rate,
            time_remaining - 1,
            best_so_far,
            max_score_estimates,
            path,
            h_free_at,
            time_remaining - 2,
        );

        moved = true;

        //path.pop();
    }

    if h_free_at >= time_remaining {
        let cave = caves.get(h_current).unwrap();
        for neighbour in cave.connected.iter() {
            if time_remaining < (neighbour.1 * 2) {
                // impossible move
                continue;
            }

            if open.contains(&neighbour.0) {
                // pointless move
                continue;
            }
            //path.push(format!("{time_remaining} h move to {neighbour:?}"));

            cave_dfs(
                caves,
                &neighbour.0,
                e_current,
                open.clone(),
                score,
                flow_rate,
                time_remaining - 1,
                best_so_far,
                max_score_estimates,
                path,
                time_remaining - (neighbour.1 * 2),
                e_free_at,
            );
            moved = true;
            //path.pop();
        }
    }
    if e_free_at >= time_remaining {
        let cave = caves.get(e_current).unwrap();
        for neighbour in cave.connected.iter() {
            if time_remaining < (neighbour.1 * 2) {
                // impossible move
                continue;
            }

            if open.contains(&neighbour.0) {
                // pointless move
                continue;
            }
            //path.push(format!("{time_remaining} e move to {neighbour:?}"));

            cave_dfs(
                caves,
                h_current,
                &neighbour.0,
                open.clone(),
                score,
                flow_rate,
                time_remaining - 1,
                best_so_far,
                max_score_estimates,
                path,
                h_free_at,
                time_remaining - (neighbour.1 * 2),
            );
            moved = true;
            //path.pop();
        }
    }

    if !moved {
        // ran out of moves, let's jump to the end?
        cave_dfs(
            caves,
            h_current,
            e_current,
            open,
            score,
            flow_rate,
            time_remaining - 1,
            best_so_far,
            max_score_estimates,
            path,
            h_free_at,
            e_free_at,
        );
    }
}

fn calc_max_scores(caves: &HashMap<String, Node>) -> HashMap<u32, u32> {
    // assume we open every valve in perfect order with 1 min between each opening

    let mut flow_rates = caves.values().map(|n| n.flow_rate).collect::<Vec<u32>>();

    flow_rates.sort();
    println!("{flow_rates:?}");
    let flow_rates = flow_rates;

    let mut output = HashMap::new();

    for t in 0..=31 {
        let mut flow_rate = 0;
        let mut score = 0;

        let mut rates = flow_rates.iter().rev();
        for _ in t..=30 {
            flow_rate += rates.next().unwrap_or(&0);
            score += flow_rate;
        }
        output.insert(t, score);
    }

    output
}

fn calc_dist(
    caves: &HashMap<String, Node>,
    start: &str,
    dest: &str,
    visited: &mut HashSet<String>,
) -> u32 {
    if start == dest {
        return 0;
    }

    let mut best = 100000;
    for (n, _) in caves.get(start).unwrap().connected.iter() {
        if visited.contains(n) {
            continue;
        }

        visited.insert(n.clone());
        let dist = 1 + calc_dist(caves, n, dest, visited);
        visited.remove(n);
        best = std::cmp::min(best, dist);
    }

    best
}

fn simplify_caves(caves: HashMap<String, Node>) -> HashMap<String, Node> {
    let mut nodes_with_flow = caves
        .iter()
        .filter(|(_, node)| node.flow_rate > 0)
        .map(|(name, _)| name)
        .cloned()
        .collect::<Vec<String>>();

    nodes_with_flow.push(String::from("AA"));

    let mut output = HashMap::new();

    for n in nodes_with_flow.iter() {
        let mut connected = Vec::new();
        for dest in nodes_with_flow.iter() {
            if n == dest {
                continue;
            }
            connected.push((
                String::from(dest),
                calc_dist(&caves, n, dest, &mut HashSet::new()),
            ));
        }

        output.insert(
            n.clone(),
            Node {
                flow_rate: caves.get(n).unwrap().flow_rate,
                connected,
            },
        );
    }

    output
}

pub fn part_one(input: &str) -> Option<u32> {
    // more than 1315

    let caves = parse(input);

    println!("{caves:?}");

    let mut caves = simplify_caves(caves);

    caves.insert(
        String::from("Nope"),
        Node {
            flow_rate: 0,
            connected: Vec::new(),
        },
    );

    println!("{caves:?}");

    let mut best_so_far = 0;

    cave_dfs(
        &caves,
        "AA",
        "Nope",
        HashSet::new(),
        0,
        0,
        60,
        &mut best_so_far,
        &calc_max_scores(&caves),
        &mut Vec::new(),
        60,
        60,
    );

    Some(best_so_far)
}

pub fn part_two(input: &str) -> Option<u32> {
    let caves = parse(input);

    println!("{caves:?}");

    let caves = simplify_caves(caves);

    println!("{caves:?}");

    let mut best_so_far = 1700;

    cave_dfs(
        &caves,
        "AA",
        "AA",
        HashSet::new(),
        0,
        0,
        26 * 2,
        &mut best_so_far,
        &calc_max_scores(&caves),
        &mut Vec::new(),
        26 * 2,
        26 * 2,
    );

    Some(best_so_far)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);

    // 2679 < x < 3100
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
