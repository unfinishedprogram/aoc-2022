pub struct Crates {
    pub stacks: Vec<Vec<char>>,
}

#[derive(Debug)]
pub struct MoveAction {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

pub fn parse_actions(input: &str) -> Vec<MoveAction> {
    input
        .lines()
        .skip(10)
        .map(|line| {
            let split = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let count: usize = split[1].parse().unwrap();
            let from: usize = split[3].parse().unwrap();
            let to: usize = split[5].parse().unwrap();
            MoveAction { count, from, to }
        })
        .collect()
}

impl Crates {
    fn parse_line(line: &str) -> Vec<Option<char>> {
        let mut res = vec![];
        for i in 0..9 {
            let ch = line.chars().nth((4 * i) + 1).unwrap();
            res.push(if ch != ' ' { Some(ch) } else { None });
        }
        res
    }

    pub fn new(input: &str) -> Self {
        let mut stacks = vec![];
        for _ in 0..9 {
            stacks.push(vec![])
        }

        let mut lines: Vec<&str> = input.lines().take(8).collect();

        lines.reverse();

        for line in lines {
            for (i, item) in Self::parse_line(line).iter().enumerate() {
                if let Some(item) = item {
                    stacks[i].push(*item)
                }
            }
        }
        Self { stacks }
    }

    pub fn apply_action_9000(&mut self, action: &MoveAction) {
        let MoveAction { count, from, to } = *action;

        for _ in 0..count {
            let from_val = self.stacks[from - 1].pop().unwrap();
            self.stacks[to - 1].push(from_val);
        }
    }

    pub fn apply_action_9001(&mut self, action: &MoveAction) {
        let MoveAction { count, from, to } = *action;
        let len = self.stacks[from - 1].len();
        let mut to_move: Vec<char> = self.stacks[from - 1]
            .splice(len - count..len, None)
            .collect();

        self.stacks[to - 1].append(&mut to_move);
    }

    pub fn as_str(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect()
    }
}

fn main() {
    let input = include_str!("../assets/day5.txt");
    {
        // Part 1
        let mut crates = Crates::new(input);
        let actions = parse_actions(input);
        for action in actions {
            crates.apply_action_9000(&action);
        }

        println!("p1:{}", crates.as_str());
    }

    {
        // Part 2
        let mut crates = Crates::new(input);
        let actions = parse_actions(input);
        for action in actions {
            crates.apply_action_9001(&action);
        }
        println!("p2:{}", crates.as_str());
    }
}
