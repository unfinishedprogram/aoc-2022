use std::collections::HashSet;

pub fn first_n_unique(input: &str, n: usize) -> usize {
    for i in 0..input.len() - n {
        if HashSet::<u8>::from_iter(input[i..i + n].as_bytes().iter().cloned()).len() == n {
            return i + n;
        }
    }
    unreachable!();
}

pub fn main() {
    let input = include_str!("../assets/day6.txt");
    println!("p1:{}", first_n_unique(input, 4));
    println!("p2:{}", first_n_unique(input, 14));
}
