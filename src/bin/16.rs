use std::backtrace::Backtrace;
use std::collections::{HashMap, HashSet, VecDeque};

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

#[derive(Debug)]
struct State {
    current: String,
    open: HashSet<String>,
    score: u32,
    flow_rate: u32,
}

fn cave_dfs(
    caves: &HashMap<String, Node>,
    current: &str,
    open: HashSet<String>,
    score: u32,
    flow_rate: u32,
    time_remaining: u32,
    best_so_far: &mut u32,
    max_score_estimates: &HashMap<u32, u32>,
    path: &mut Vec<String>,
) {
    if time_remaining == 0 {
        if score > *best_so_far {
            *best_so_far = score;
            println!("{current} {score} {flow_rate} {open:?} {path:?}");
        }
        return;
    }

    let score = score + flow_rate;

    if score + max_score_estimates.get(&(30 - time_remaining)).unwrap() <= *best_so_far {
        // Pointless avenue
        return;
    }

    let cave = caves.get(current).unwrap();

    let mut moved = false;

    for neighbour in cave.connected.iter() {
        //path.push(format!("move to {neighbour}"));
        if time_remaining < neighbour.1 {
            // impossible move
            continue;
        }

        if open.contains(&neighbour.0) {
            // pointless move
            continue;
        }

        cave_dfs(
            caves,
            &neighbour.0,
            open.clone(),
            score + (neighbour.1 - 1) * flow_rate,
            flow_rate,
            time_remaining - neighbour.1,
            best_so_far,
            max_score_estimates,
            path,
        );
        moved = true;
        //path.pop();
    }

    if !open.contains(current) {
        let mut new_open = open.clone();
        new_open.insert(current.to_string());

        //path.push(format!("open {current}"));
        cave_dfs(
            caves,
            current,
            new_open,
            score,
            flow_rate + cave.flow_rate,
            time_remaining - 1,
            best_so_far,
            max_score_estimates,
            path,
        );

        moved = true;

        //path.pop();
    }

    if !moved {
        // ran out of moves, let's jump to the end?
        cave_dfs(
            caves,
            current,
            open,
            score,
            flow_rate,
            time_remaining - 1,
            best_so_far,
            max_score_estimates,
            path,
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

    for t in 0..=30 {
        let mut flow_rate = 0;
        let mut score = 0;

        let mut rates = flow_rates.iter().rev();
        for step in t..=30 {
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
        .filter(|(name, node)| node.flow_rate > 0)
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

    let caves = simplify_caves(caves);

    println!("{caves:?}");

    let mut best_so_far = 1650;

    cave_dfs(
        &caves,
        "AA",
        HashSet::new(),
        0,
        0,
        30,
        &mut best_so_far,
        &calc_max_scores(&caves),
        &mut Vec::new(),
    );

    Some(best_so_far)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
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
        assert_eq!(part_two(&input), None);
    }
}
