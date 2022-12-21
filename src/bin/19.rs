#[derive(Debug)]
struct Blueprint {
    ore_robot: u32,
    clay_robot: u32,
    obs_robot: (u32, u32),
    geo_robot: (u32, u32),
}

impl Blueprint {
    fn parse(input: &str) -> Blueprint {
        // Blueprint 1:
        //   Each ore robot costs 4 ore.
        //   Each clay robot costs 4 ore.
        //   Each obsidian robot costs 4 ore and 14 clay.
        //   Each geode robot costs 3 ore and 16 obsidian.

        let (_, rest) = input.split_once(": Each ore robot costs ").unwrap();

        let (ore, rest) = rest.split_once(" ore. Each clay robot costs ").unwrap();

        let (clay, rest) = rest.split_once(" ore. Each obsidian robot costs ").unwrap();
        let (obs_ore, rest) = rest.split_once(" ore and ").unwrap();
        let (obs_clay, rest) = rest.split_once(" clay. Each geode robot costs ").unwrap();
        let (geo_ore, rest) = rest.split_once(" ore and ").unwrap();
        let (geo_obs, _) = rest.split_once(" obsidian.").unwrap();

        Blueprint {
            ore_robot: ore.parse::<u32>().unwrap(),
            clay_robot: clay.parse::<u32>().unwrap(),
            obs_robot: (
                obs_ore.parse::<u32>().unwrap(),
                obs_clay.parse::<u32>().unwrap(),
            ),
            geo_robot: (
                geo_ore.parse::<u32>().unwrap(),
                geo_obs.parse::<u32>().unwrap(),
            ),
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,

    ore_robot: u32,
    clay_robot: u32,
    obs_robot: u32,
    geo_robot: u32,
}

impl State {
    fn new() -> State {
        State {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,

            ore_robot: 1,
            clay_robot: 0,
            obs_robot: 0,
            geo_robot: 0,
        }
    }
    fn tick(&self) -> State {
        State {
            ore: self.ore + self.ore_robot,
            clay: self.clay + self.clay_robot,
            obsidian: self.obsidian + self.obs_robot,
            geode: self.geode + self.geo_robot,

            ore_robot: self.ore_robot,
            clay_robot: self.clay_robot,
            obs_robot: self.obs_robot,
            geo_robot: self.geo_robot,
        }
    }
}

fn simulate(bp: &Blueprint, time: u32, state: State, count: &mut u64) -> u32 {
    *count += 1;

    if *count % 100000000 == 0{
        println!("{count} {time} {state:?} ");
    }

    if time == 0 {
        return state.geode;
    }

    //println!("{time} {state:?}");

    let mut max = 0;

    // if can make ore_robot, try that
    if state.ore >= bp.ore_robot {
        let mut state = state.tick();
        state.ore_robot += 1;
        state.ore -= bp.ore_robot;
        max = std::cmp::max(max, simulate(bp, time - 1, state, count));
    }

    // if can make clay_robot, try that
    if state.ore >= bp.clay_robot {
        let mut state = state.tick();
        state.clay_robot += 1;
        state.ore -= bp.clay_robot;
        max = std::cmp::max(max, simulate(bp, time - 1, state, count));
    }

    // if can make obs robot, try that
    if state.ore >= bp.obs_robot.0 && state.clay >= bp.obs_robot.1 {
        let mut state = state.tick();
        state.obs_robot += 1;
        state.ore -= bp.obs_robot.0;
        state.clay -= bp.obs_robot.1;
        max = std::cmp::max(max, simulate(bp, time - 1, state, count));
    }

    // if can make geo robot, try that. If this is possible it always makes sense to do?
    if state.ore >= bp.geo_robot.0 && state.obsidian >= bp.geo_robot.1 {
        let mut state = state.tick();
        state.geo_robot += 1;
        state.ore -= bp.geo_robot.0;
        state.obsidian -= bp.geo_robot.1;
        max = std::cmp::max(max, simulate(bp, time - 1, state, count));
    } else {
        // try doing nothing too

        let state = state.tick();
        max = std::cmp::max(max, simulate(bp, time - 1, state, count));
    }

    max
}

fn parse(input: &str) -> Vec<Blueprint> {
    input.lines().map(Blueprint::parse).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let bps = parse(input);

    println!("{bps:?}");
    /*Some(bps.iter().enumerate().map(|(idx, bp)| {
        let sim = simulate(bp, 24, State::new(), &mut 0);
        println!("{idx} {sim}");
        (idx as u32 + 1) * sim
    }).sum::<u32>()) */

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let bps = parse(input);

    println!("{bps:?}");
    Some(bps[0..1].iter().fold(1, |prev, bp| {
        let sim = simulate(bp, 32, State::new(), &mut 0);
        println!("{prev} {sim}");
        prev * sim
    }))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), None);
    }
}
