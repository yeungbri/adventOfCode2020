use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    // return part1();
    return part2();
}

fn part1() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;
    for line in reader.lines() {
        let line = match line {
            Ok(s) => s,
            Err(e) => return Err(e),
        };
        // println!("{}", line?);
        let vec: Vec<&str> = line.split(" ").collect();
        let range: Vec<&str> = vec[0].split("-").collect();
        let min = range[0].parse::<usize>().unwrap();
        let max = range[1].parse::<usize>().unwrap();

        let req_letter= &vec[1][..1];
        let password = vec[2];
        let cnt = password.matches(req_letter).count();
        // println!("{} {} {} {} {}", min, max, req_letter, password, cnt);

        if cnt <= max && cnt >= min {
            result += 1;
        }
    }

    println!("{}", result);

    Ok(())
}

fn part2() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;
    for line in reader.lines() {
        let line = match line {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        let vec: Vec<&str> = line.split(" ").collect();

        // 1 indexed
        let two_pos: Vec<&str> = vec[0].split("-").collect();
        let pos1 = two_pos[0].parse::<usize>().unwrap();
        let pos2 = two_pos[1].parse::<usize>().unwrap();

        let req_letter = &vec[1][..1];
        let password = vec[2];

        let correct_pos1: bool = match password.chars().nth(pos1 - 1) {
            None => false,
            Some(x) => x.to_string() == req_letter,
        };
        let correct_pos2: bool = match password.chars().nth(pos2 - 1) {
            None => false,
            Some(x) => x.to_string() == req_letter,
        };
        // println!("{} {} {} {}", correct_pos1, correct_pos2, password, req_letter);

        if (correct_pos1 && !correct_pos2) || (!correct_pos1 && correct_pos2) {
            println!("{} {} {} {} {} {}", correct_pos1, correct_pos2, password, req_letter, pos1, pos2);
            result += 1;
        }
    }

    println!("{}", result);

    Ok(())
}
