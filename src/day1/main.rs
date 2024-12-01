use std::collections::HashMap;
use std::io::{self};
fn parse_file() -> io::Result<(Vec<i32>, Vec<i32>)> {
    // Embed content in binary
    let contents = include_str!("../../puzzles/day1-hard.txt"); 

    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
            if let (Ok(num1), Ok(num2)) = (first.parse::<i32>(), second.parse::<i32>()) {
                column1.push(num1);
                column2.push(num2);
            }
        }
    }

    Ok((column1, column2))
}

pub fn main() {
    let (mut column1, mut column2) = parse_file().unwrap();

    column1.sort_unstable();
    column2.sort_unstable();

    let output: i32 = column1.iter().zip(column2.iter()).map(|(a, b)| (a - b).abs()).sum();
    println!("Day1 Part1: {}", output);

    let mut counts = HashMap::new();
    for &num in &column2 {
        *counts.entry(num).or_insert(0) += 1;
    }

    let output2: i32 = column1.iter()
        .map(|&num| counts.get(&num).map_or(0, |&count| num * count as i32))
        .sum();

    println!("Day1 Part2: {}", output2);
}
