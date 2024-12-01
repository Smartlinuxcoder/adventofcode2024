use std::fs::File;
use std::io::{self, BufRead};
use rayon::prelude::*;

fn parse_file() -> io::Result<(Vec<i32>, Vec<i32>)> {
    let path = "puzzles/day1.txt";

    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut column1: Vec<i32> = Vec::with_capacity(1000);
    let mut column2: Vec<i32> = Vec::with_capacity(1000);

    for line in reader.lines() {
        let line = line?; 

        let parts: Vec<&str> = line.split_whitespace().collect();
        if let (Some(&first), Some(&second)) = (parts.get(0), parts.get(1)) {
            if let (Ok(num1), Ok(num2)) = (first.parse::<i32>(), second.parse::<i32>()) {
                column1.push(num1);
                column2.push(num2);
            }
        }
    }

    Ok((column1, column2))
}

pub fn main() {
    let (column1, column2) = parse_file().unwrap();
    let mut sorted1 = column1.clone();
    sorted1.par_sort_unstable();
    let mut sorted2 = column2.clone();
    sorted2.par_sort_unstable();
    let mut output = 0;
    for i in 0..sorted1.len() {
        output += (sorted1[i] - sorted2[i]).abs();
    }

    println!("Day1 Part1: {}", output.abs());


    let mut scores: Vec<(usize, usize)> = Vec::new();

    for i in 0..column1.len() {
        let count2 = sorted2.iter().filter(|&&x| x == column1[i]).count();
        scores.push( (column1[i].try_into().unwrap(), count2));
    }
    let mut output2 = 0;
    for (key, value) in scores.iter() {
        output2 += key * value;
    }
    println!("Day1 Part2: {}", output2);
}
