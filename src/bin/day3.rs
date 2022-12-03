#![feature(iter_array_chunks)]

use std::{collections::hash_map::RandomState, collections::HashSet, ops::BitAnd};

fn get_priority(ch: char) -> i32 {
    let byte = ch as u8;
    match byte {
        65..=90 => ch as i32 - 64 + 26 as i32, // A-Z
        97..=122 => ch as i32 - 96 as i32,     // a-z
        _ => unreachable!(),
    }
}

fn separate_halves(s: &str) -> Vec<String> {
    let half = s.len() / 2;
    let first: String = s[..half].chars().collect();
    let second: String = s[half..].chars().collect();
    vec![first, second]
}

fn get_overlap(between: Vec<String>) -> char {
    type HS = HashSet<char, RandomState>;
    let mut as_sets = between.iter().map(|s| HS::from_iter(s.chars()));
    let first = as_sets.next().unwrap();
    let overlap: HS = as_sets.fold(first, |acc: HS, set: HS| set.bitand(&acc));
    *overlap.iter().next().unwrap()
}

fn main() {
    let input = include_str!("../assets/day3.txt");

    let p1: i32 = input
        .lines()
        .map(separate_halves)
        .map(get_overlap)
        .map(get_priority)
        .sum();

    let p2: i32 = input
        .lines()
        .array_chunks::<3>()
        .map(|v| Vec::from(v.map(str::to_owned)))
        .map(get_overlap)
        .map(get_priority)
        .sum();

    println!("p1:{p1}");
    println!("p2:{p2}")
}
