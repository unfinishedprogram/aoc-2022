fn main() {
    let input = include_str!("../assets/day1.txt");

    let mut elves: Vec<i32> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|l| l.parse::<i32>().unwrap()).sum())
        .collect::<Vec<i32>>();

    elves.sort();

    println!("p1:{}", elves.iter().max().unwrap());
    println!("p2:{}", elves.iter().rev().take(3).sum::<i32>());
}
