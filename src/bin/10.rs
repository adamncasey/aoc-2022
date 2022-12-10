#[derive(Debug)]
enum Instr {
    Noop,
    Addx(i32),
}

impl Instr {
    fn parse(input: &str) -> Instr {
        match input.split_once(" ") {
            Some(("addx", num)) => Instr::Addx(num.parse::<i32>().unwrap()),
            None if input == "noop" => Instr::Noop,
            _ => panic!("unexpected input: {input}"),
        }
    }

    fn cycles(&self) -> usize {
        match self {
            Instr::Noop => 0,
            Instr::Addx(_) => 1,
        }
    }

    fn complete(&self, register: &mut i32) {
        match self {
            Instr::Noop => {}
            Instr::Addx(val) => *register += val,
        }
    }
}

fn calc(input: &str) -> (i32, String) {
    let mut instrs = input.lines().map(Instr::parse).collect::<Vec<Instr>>();
    instrs.reverse();

    let mut result: i32 = 0;
    let mut display = String::from("");

    let mut register: i32 = 1;
    let mut next_work: Option<(usize, Instr)> = None;
    for cycle in 1..=236 {
        if let None = next_work {
            let instr = instrs.pop().unwrap();
            next_work = Some((cycle + instr.cycles(), instr));
        }

        // if cycle is important, record result

        if cycle > 19 && (cycle - 20) % 40 == 0 {
            result += cycle as i32 * register;
        }

        let col = ((cycle - 1) % 40) as i32;

        if col == 0 {
            display.push('\n');
        }

        let ch = if col > register + 1 || col < register - 1 {
            ' '
        } else {
            '#'
        };
        display.push(ch);

        //println!("{cycle}: {register} {next_work:?} {result} {col} {ch:?}");

        // change register if execution finished
        if let Some((finish_cycle, instr)) = &next_work {
            if cycle == *finish_cycle {
                instr.complete(&mut register);
                next_work = None;
            }
        }
    }

    (result, display)
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(calc(input).0)
}

pub fn part_two(input: &str) -> Option<String> {
    Some((calc(input).1[..]).into())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let _ = r#"
        noop
addx 3
addx -5"#; //
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(
            part_two(&input),
            Some(String::from(
                r#"
##  ##  ##  ##  ##  ##  ##  ##  ##  ##  
###   ###   ###   ###   ###   ###   ### 
####    ####    ####    ####    ####    
#####     #####     #####     #####     
######      ######      ######      ####
#######       #######       ####### "#
            ))
        );
    }
}
