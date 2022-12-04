type IdRange = (i32, i32);

pub fn parse_line(line: &str) -> (IdRange, IdRange) {
    let mut split = line.split(",");
    (
        parse_range(split.next().unwrap()),
        parse_range(split.next().unwrap()),
    )
}

pub fn parse_range(range: &str) -> IdRange {
    let mut split = range.split("-");
    (
        split.next().unwrap().parse::<i32>().unwrap(),
        split.next().unwrap().parse::<i32>().unwrap(),
    )
}

pub fn inside_bounds(pair: (IdRange, IdRange)) -> bool {
    let ((a_min, a_max), (b_min, b_max)) = pair;
    (a_min >= b_min && a_max <= b_max) || (b_min >= a_min && b_max <= a_max)
}

pub fn some_inside_bounds(pair: (IdRange, IdRange)) -> bool {
    let ((a_min, a_max), (b_min, b_max)) = pair;

    for i in a_min..=a_max {
        for j in b_min..=b_max {
            if i == j {
                return true;
            }
        }
    }

    false
}

fn main() {
    let input = include_str!("../assets/day4.txt");

    let p1 = input
        .lines()
        .map(parse_line)
        .filter(|a| inside_bounds(*a))
        .count();

    let p2 = input
        .lines()
        .map(parse_line)
        .filter(|a| some_inside_bounds(*a))
        .count();

    println!("p2:{p1}");
    println!("p2:{p2}");
}
