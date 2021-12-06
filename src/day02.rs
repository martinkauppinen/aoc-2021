enum Direction {
    Forward,
    Down,
    Up,
}

struct Command {
    dir: Direction,
    amount: usize,
}

struct Submarine {
    position: usize,
    depth: usize,
    aim: usize,
}

impl Submarine {
    pub fn new() -> Self {
        Self {
            position: 0,
            depth: 0,
            aim: 0,
        }
    }

    pub fn result(&self) -> usize {
        self.position * self.depth
    }

    pub fn command(&mut self, cmd: Command) {
        match cmd.dir {
            Direction::Forward => self.position += cmd.amount,
            Direction::Down => self.depth += cmd.amount,
            Direction::Up => self.depth -= cmd.amount,
        }
    }

    pub fn aim(&mut self, cmd: Command) {
        match cmd.dir {
            Direction::Forward => {
                self.position += cmd.amount;
                self.depth += self.aim * cmd.amount;
            }
            Direction::Down => self.aim += cmd.amount,
            Direction::Up => self.aim -= cmd.amount,
        }
    }
}

fn parse_command(cmd: &str) -> Command {
    let mut iter = cmd.split_whitespace();
    let dir: Direction;
    let amount: usize;
    match iter.next().unwrap() {
        "forward" => dir = Direction::Forward,
        "down" => dir = Direction::Down,
        "up" => dir = Direction::Up,
        _ => {
            panic!("Something went wrong")
        }
    };
    amount = iter.next().unwrap().parse().unwrap();
    Command { dir, amount }
}

fn part1(lines: &[String]) -> usize {
    let mut sub = Submarine::new();
    let cmds: Vec<Command> = lines.iter().map(|line| parse_command(line)).collect();
    for cmd in cmds {
        sub.command(cmd);
    }
    sub.result()
}

fn part2(lines: &[String]) -> usize {
    let mut sub = Submarine::new();
    let cmds: Vec<Command> = lines.iter().map(|line| parse_command(line)).collect();
    for cmd in cmds {
        sub.aim(cmd);
    }
    sub.result()
}

pub fn run(lines: &[String]) {
    run_parts!(lines);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> Vec<String> {
        vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_test_input()), 150);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_test_input()), 900);
    }
}
