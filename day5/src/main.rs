use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    // return part1();
    return part2();
}

fn binary_space_parition(code: Vec<char>) -> (i32, i32) {
    let mut lower = 0;
    let mut upper = 127;

    let row;
    for c in code[..6].iter() {
        if *c == 'F' {
            upper = lower + (upper - lower) / 2;
        } else {
            lower = upper - (upper - lower) / 2;
        }
        // println!("{} {} {}", c, lower, upper);
    }
    if code[6] == 'F' {
        row = lower;
    } else {
        row = upper;
    }
    // println!("{} {}", code[6], row);

    lower = 0;
    upper = 7;
    let col;
    for c in code[7..9].iter() {
        if *c == 'L' {
            upper = lower + (upper - lower) / 2;
        } else {
            lower = upper - (upper - lower) / 2;
        }
        // println!("{} {} {}", c, lower, upper);
    }
    if code[9] == 'L' {
        col = lower;
    } else {
        col = upper;
    }
    // println!("{} {}", code[9], col);

    (row, col)
}

fn part1() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;
    for line in reader.lines() {
        // println!("{}", line?);
        let char_vec: Vec<char> = line?.chars().collect();
        let (row, col) = binary_space_parition(char_vec);
        let id = row * 8 + col;
        
        if id > result {
            result = id;
        }
        // println!("---");
    }

    println!("{}", result);

    Ok(())
}

fn part2() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut ids: Vec<i32> = Vec::new();
    for line in reader.lines() {
        // println!("{}", line?);
        let char_vec: Vec<char> = line?.chars().collect();
        let (row, col) = binary_space_parition(char_vec);
        let id = row * 8 + col;
        ids.push(id);
    }

    ids.sort();
    for (i, x) in ids.iter().enumerate() {
        println!("{}", x);
        if i != 0 {
            if x - 1 != ids[i-1] {
                println!("answer: {}", x - 1);
                break;
            }
        }
    }

    Ok(())
}
