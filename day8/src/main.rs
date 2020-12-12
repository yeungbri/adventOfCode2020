use std::fs::File;
use std::io::{self, prelude::*};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    // return part1();
    return part2();
}

fn part1() -> io::Result<()> {
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    let program: Vec<&str> = contents.split("\n").collect();
    // println!("{}", program.get(1).unwrap());
    
    let mut acc = 0;
    let mut cur: i32 = 0;
    let mut history: HashSet<i32> = HashSet::new();
    loop {
        // check if seen before
        if history.contains(&cur) {
            break
        }
        history.insert(cur);

        let line = program.get(cur as usize).unwrap();
        let parsed: Vec<&str> = line.split(" ").collect();
        let val = parsed[1].trim().parse::<i32>().unwrap();

        match &line[..3] {
            "nop" => {
                cur += 1;
            },
            "acc" => {
                acc += val;
                cur += 1;
            },
            "jmp" => {
                cur += val;
            },
            _ => ()
        }
    }

    println!("{}", acc);

    Ok(())
}

// either one jmp is supposed to be a nop or one nop is supposed to be a jmp
fn part2() -> io::Result<()> {
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    let program: Vec<&str> = contents.split("\n").collect();
    // println!("{}", program.get(1).unwrap());
    
    let mut result: Option<i32> = None;
    let mut mods: HashSet<i32> = HashSet::new();
    let mut should_mod = false;
    loop {
        if result.is_some() {
            break;
        }

        let mut acc = 0;
        let mut cur: i32 = 0;
        let mut history: HashSet<i32> = HashSet::new();
        loop {
            if cur as usize >= program.len() {
                result = Some(acc);
                break;
            } else if history.contains(&cur) {
                break;
            }
            history.insert(cur);

            let line = program.get(cur as usize).unwrap();
            let parsed: Vec<&str> = line.split(" ").collect();
            let val = parsed[1].trim().parse::<i32>().unwrap();

            match &line[..3] {
                "nop" => {
                    if should_mod && !mods.contains(&cur) {
                        // println!("{} {}", cur, mods.len());
                        mods.insert(cur);
                        should_mod = false;
                        // do jmp instead
                        cur += val;
                    } else {
                        cur += 1;
                    }
                },
                "acc" => {
                    acc += val;
                    cur += 1;
                },
                "jmp" => {
                    if should_mod && !mods.contains(&cur) {
                        // println!("{} {}", cur, mods.contains(&cur));
                        mods.insert(cur);
                        should_mod = false;
                        // do nop instead
                        cur += 1;
                    } else {
                        cur += val;
                    }
                },
                _ => ()
            }
        }
        // println!("{} {:?}", mods.len(), mods);

        should_mod = true;
    }

    println!("{}", result.unwrap());
    Ok(())
}
