
use anyhow::Result;
use regex::Regex;

pub fn aoc3_1() -> Result<()> {
    let src = std::fs::read_to_string("input/aoc3.txt")?;

    let re = Regex::new(r"mul\(\d+,\d+\)")?;

    let mut sum = 0;
    for mat in re.find_iter(&src) {
        let str = mat.as_str();
        sum += str[4..str.len()-1]
            .split(",")
            .map(|n| n.parse::<i32>().unwrap())
            .product::<i32>();
    }

    println!("(AOC 3_1) Sum of operations is: {sum}");

    Ok(())
}

pub fn aoc3_2() -> Result<()> {
    let src = std::fs::read_to_string("input/aoc3.txt")?;

    // regex that matches `mul(d+,d+)` or `do()` or `don't()`
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)")?;

    let mut cancel = false;
    let mut sum = 0;
    for mat in re.find_iter(&src) {
        let str = mat.as_str();
        match str {
            _ if str.starts_with("don") => cancel = true,
            _ if str.starts_with("do") => cancel = false,
            _ if !cancel => {
                sum += str[4..str.len()-1]
                    .split(",")
                    .map(|n| n.parse::<i32>().unwrap())
                    .product::<i32>();
            },
            _ => {}
        }
    }

    println!("(AOC 3_2) Sum of multiplications: {sum}");

    Ok(())
}

