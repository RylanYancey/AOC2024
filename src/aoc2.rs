
use anyhow::Result;

pub fn aoc2_1() -> Result<()> {
    let src = std::fs::read_to_string("input/aoc2.txt")?;

    let mut nums = Vec::new();
    let mut safe = 0;

    for line in src.lines() {
        // collect numbers into a buffer
        nums.clear();
        line.split(" ").map(|n| n.parse::<i32>().unwrap()).collect_into(&mut nums);        

        if !report_is_safe(&nums) {
            continue;
        }

        safe += 1;
    }

    println!("(AOC 2_1) Number of safe reports: {safe}");

    Ok(())
}

pub fn aoc2_2() -> Result<()> {
    let src = std::fs::read_to_string("input/aoc2.txt")?;

    let mut nums = Vec::new();
    let mut safe = 0;

    for line in src.lines() {
        // collect numbers into a buffer
        nums.clear();
        line.split(" ").map(|n| n.parse::<i32>().unwrap()).collect_into(&mut nums);      

        // if the report can be made safe by
        // removing a number, it still counts as safe.
        if !report_is_safe(&nums) {
            let mut without1 = Vec::new();

            'inner:
            for i in 0..nums.len() {
                nums.clone_into(&mut without1);
                without1.remove(i);

                if report_is_safe(&without1) {
                    safe += 1;
                    break 'inner;
                }
            }
        } else {
            safe += 1;
        }
    }
    
    println!("(AOC 2_2) Number of safe reports: {safe}");

    Ok(())
}

fn report_is_safe(nums: &[i32]) -> bool {
    // determine if there are any invalid adjacentcies.
    for i in 1..nums.len()-1 {
        let d1 = (nums[i] - nums[i-1]).abs();
        let d2 = (nums[i] - nums[i+1]).abs();

        if !(1..=3).contains(&d1) || !(1..=3).contains(&d2) {
            return false;
        }
    }

    // determine if all numbers are ascending or descending.
    let dir = (nums[0] - nums[1]).signum();
    for i in 0..nums.len()-1 {
        if (nums[i] - nums[i+1]).signum() != dir {
            return false;
        }
    }

    true
}