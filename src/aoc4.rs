
use anyhow::Result;

const DIRS: [(i8, i8); 8] = [(1, 1), (-1, -1), (-1, 1), (1, -1), (0, 1), (1, 0), (0, -1), (-1, 0)]; 

pub fn aoc4_1() -> Result<()> {
    let src = std::fs::read_to_string("input/aoc4.txt")?;
    let cols = src.lines().next().unwrap().len();
    let buffer: Vec<u8> = src.as_bytes().iter().copied().filter(|b| *b != b'\n').collect();
    let rows = buffer.len() / cols;

    let mut sum = 0;
    for row in 0..rows {
        for col in 0..cols {
            if buffer[row + col * cols] == b'X' {
                for dir in DIRS {
                    let mut iter = SquareIter::new((row, col), (rows, cols), dir);
                    if  iter.next().is_some_and(|index| buffer[index] == b'M') &&
                        iter.next().is_some_and(|index| buffer[index] == b'A') &&
                        iter.next().is_some_and(|index| buffer[index] == b'S') {
                            sum += 1;
                        }
                }
            }
        }
    }

    println!("(AOC4_1) Total number of 'XMAS': {sum}.");

    Ok(())
}

struct SquareIter {
    bounds: (i32, i32),
    curr: (i32, i32),
    dir: (i32, i32),
}

impl SquareIter {
    fn new(start: (usize, usize), bounds: (usize, usize), dir: (i8, i8)) -> Self {
        Self {
            bounds: (bounds.0 as i32, bounds.1 as i32),
            curr: (start.0 as i32, start.1 as i32),
            dir: (dir.0 as i32, dir.1 as i32)
        }
    }
}

impl Iterator for SquareIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.curr.0 + self.dir.0;
        let col = self.curr.1 + self.dir.1;
        self.curr = (row, col);

        if (0..self.bounds.0).contains(&row) && (0..self.bounds.1).contains(&col) {
            Some((row + col * self.bounds.1) as usize)
        } else {
            None
        }
    }
}

pub fn aoc4_2() -> Result<()> {
    let src = std::fs::read_to_string("input/aoc4.txt")?;
    let cols = src.lines().next().unwrap().len();
    // Convert the string into a matrix grid.
    let buffer: Vec<u8> = src.as_bytes().iter().copied().filter(|b| *b != b'\n').collect();
    let rows = buffer.len() / cols;

    let mut sum = 0;
    for row in 0..rows {
        'inner:
        for col in 0..cols {
            if buffer[row + col * cols] == b'A' {
                for dir in [(1, 1), (1, -1)] {
                    let rev = (-dir.0, -dir.1);
                    match [try_shift((row, col), dir, (rows, cols)), try_shift((row, col), rev, (rows, cols))] {
                        [Some(i), Some(j)] => {
                            if !(buffer[i] == b'M' && buffer[j] == b'S' || buffer[i] == b'S' && buffer[j] == b'M') {
                                continue 'inner;
                            }
                        },
                        _ => continue 'inner
                    }
                }

                sum += 1;
            }
        }
    }

    println!("(AOC4_2) Total number of X-MAS: {sum}.");

    Ok(())
}

fn try_shift(from: (usize, usize), dir: (i32, i32), bounds: (usize, usize)) -> Option<usize> {
    let new = (from.0 as i32 + dir.0, (from.1 as i32 + dir.1));
    if (0..bounds.0 as i32).contains(&new.0) && (0..bounds.1 as i32).contains(&new.1) {
        Some(new.0 as usize + new.1 as usize * bounds.1)
    } else {
        None
    }
}