
use std::{cmp::Ordering, collections::HashMap};
use anyhow::Result;

pub fn aoc5_1() -> Result<()> {
    let src = std::fs::read_to_string("input/aoc5.txt")?;
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    src.lines()
        .take_while(|line| *line != "")
        .map(|line| line.split("|").map(|n| n.parse::<i32>().unwrap()))
        .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
        .for_each(|(x, y)| rules.entry(x).or_insert_with(|| Vec::new()).push(y));

    let sum = src.lines()
        .skip_while(|line| line.contains("|") || *line == "")
        .map(|line| line.split(",").map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>())
        .fold(0, |sum, nums: Vec<i32>| 
            if nums.is_sorted_by(|a, b| !rules.get(b).is_some_and(|v| v.contains(a))) { sum + nums[nums.len() / 2] } 
            else { sum }
        );

    println!("(AOC 5-1) Sum of middles: {sum}.");
    Ok(())
}

pub fn aoc5_2() -> Result<()> {
    let src = std::fs::read_to_string("input/aoc5.txt")?;
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    src.lines()
        .take_while(|line| *line != "")
        .map(|line| line.split("|").map(|n| n.parse::<i32>().unwrap()))
        .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
        .for_each(|(x, y)| rules.entry(x).or_insert_with(|| Vec::new()).push(y));

    let sum = src.lines()
        .skip_while(|line| line.contains("|") || *line == "")
        .map(|line| line.split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>())
        .fold(0, |sum, mut nums: Vec<i32>| 
            if !nums.is_sorted_by(|a, b| !rules.get(b).is_some_and(|v| v.contains(a))) { 
                nums.sort_by(|a, b| 
                    if rules.get(a).is_some_and(|v| v.contains(b)) { Ordering::Less } 
                    else if rules.get(b).is_some_and(|v| v.contains(a)) { Ordering::Greater } 
                    else { Ordering::Equal }
                ); 
                sum + nums[nums.len() / 2] 
            } else { 
                sum 
            }
        );
    println!("(AOC 5-2) Sum of middles: {sum}.");
    Ok(())
}