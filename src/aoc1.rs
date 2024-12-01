
use anyhow::Result;

/// https://adventofcode.com/2024/day/1
pub fn aoc1_1() -> Result<()> {    
    let src = std::fs::read_to_string("input/aoc1.txt")?;
    
    // Fill buffers with the numbers.
    let mut left = Vec::with_capacity(2048);
    let mut right = Vec::with_capacity(2048);
    for line in src.lines() {
       left.push(line[0..5].parse::<i32>()?);
       right.push(line[8..13].parse::<i32>()?);
    }

    // sort_unstable is faster, but
    // is unsuitable for some edge cases.
    // its' fine here.
    left.sort_unstable();
    right.sort_unstable();

    // compute the sum of the differences
    let mut sum = 0;
    for (left, right) in left.iter().zip(&right) {
        sum += (left - right).abs() 
    }

    println!("(AOC1_1) The Sum of Distances is: {sum}.");
    
    Ok(())
}

pub fn aoc1_2() -> Result<()> {
    let src = std::fs::read_to_string("input/aoc1.txt")?;

    // Fill buffers with the numbers.
    let mut left = Vec::with_capacity(1024);
    let mut right = Vec::with_capacity(1024);
    for line in src.lines() {
       left.push(line[0..5].parse::<i32>()?);
       right.push(line[8..13].parse::<i32>()?);
    }
   
    let mut score = 0;
    for left in &left {
        score += left * right.iter().filter(|n| **n == *left).count() as i32;
    }

    println!("(AOC1_2) Similarity Score is: {score}.");

   Ok(()) 
}
