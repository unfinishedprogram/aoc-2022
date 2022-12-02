#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl From<Shape> for i32 {
    fn from(val: Shape) -> i32 {
        match val {
            Shape::Rock => 0,
            Shape::Paper => 1,
            Shape::Scissors => 2,
        }
    }
}

impl From<i32> for Shape {
    fn from(val: i32) -> Shape {
        match val {
            0 => Shape::Rock,
            1 => Shape::Paper,
            2 => Shape::Scissors,
            _ => unreachable!(),
        }
    }
}

enum Outcome {
    Lose,
    Tie,
    Win,
}

fn desired_outcome(game: (Shape, Outcome)) -> (Shape, Shape) {
    use Outcome::*;

    match game {
        (shape, Tie) => (shape, shape),
        (shape, Lose) => (shape, Shape::from((i32::from(shape) + 2) % 3)),
        (shape, Win) => (shape, Shape::from((i32::from(shape) + 1) % 3)),
    }
}

fn parse_desired_outcome_line(line: &str) -> (Shape, Outcome) {
    let mut split = line.split(" ");
    use Outcome::*;
    use Shape::*;

    (
        match split.next().unwrap() {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => unreachable!(),
        },
        match split.next().unwrap() {
            "X" => Lose,
            "Y" => Tie,
            "Z" => Win,
            _ => unreachable!(),
        },
    )
}

fn choice_value(choice: Shape) -> i32 {
    match choice {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn parse_line(line: &str) -> (Shape, Shape) {
    use Shape::*;
    let mut split = line.split(" ").map(|c| match c {
        "A" | "X" => Rock,
        "B" | "Y" => Paper,
        "C" | "Z" => Scissors,
        _ => unreachable!(),
    });

    (split.next().unwrap(), split.next().unwrap())
}

fn score_game(game: (Shape, Shape)) -> i32 {
    let (them, us) = game;

    let win_score = match (&us, &them) {
        (Shape::Rock, Shape::Rock) => 3,
        (Shape::Scissors, Shape::Scissors) => 3,
        (Shape::Paper, Shape::Paper) => 3,

        (Shape::Paper, Shape::Rock) => 6,
        (Shape::Rock, Shape::Scissors) => 6,
        (Shape::Scissors, Shape::Paper) => 6,

        (Shape::Paper, Shape::Scissors) => 0,
        (Shape::Scissors, Shape::Rock) => 0,
        (Shape::Rock, Shape::Paper) => 0,
    };

    win_score + choice_value(us)
}

fn score_line(line: &str) -> i32 {
    score_game(parse_line(line))
}

fn main() {
    let input = include_str!("../assets/day2.txt");

    println!("p1:{}", input.lines().map(score_line).sum::<i32>());
    println!(
        "p2:{}",
        input
            .lines()
            .map(parse_desired_outcome_line)
            .map(desired_outcome)
            .map(score_game)
            .sum::<i32>()
    );
}
