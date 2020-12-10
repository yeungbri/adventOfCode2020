use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::{HashSet, HashMap};

fn main() -> io::Result<()> {
    // return part1();
    return part2();
}

// count number of yes' that anyone answered
fn part1() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;
    let mut yesQuestions: HashSet<char> = HashSet::new();
    for line in reader.lines() {
        let line = match line {
            Ok(s) => s,
            Err(e) => "".to_string()
        };
        // println!("{}", line?);
        if line.trim().is_empty() {
            result += yesQuestions.len();
            yesQuestions.drain();
        } else {
            let char_vec: Vec<char> = line.chars().collect();
            for c in char_vec {
                yesQuestions.insert(c);
            }
        }
    }
    result += yesQuestions.len();
    yesQuestions.drain();

    println!("{}", result);

    Ok(())
}

// count number of yes' that everyone in the group answered
fn part2() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;
    let mut qa: HashMap<char, i32> = HashMap::new();
    let mut group_sz = 0;
    for line in reader.lines() {
        // println!("{}", line?);
        let line = match line {
            Ok(s) => s,
            Err(e) => "".to_string()
        };

        if line.trim().is_empty() {
            result += qa.values().fold(0, |acc, &x| {
                if x == group_sz {
                    return acc + 1
                }
                acc
            });
            qa.clear();
            group_sz = 0;
        } else {
            let char_vec: Vec<char> = line.chars().collect();
            for c in char_vec {
                qa.insert(c, qa.get(&c).unwrap_or(&0) + 1);
            }
            group_sz += 1;
        }
    }

    result += qa.values().fold(0, |acc, &x| {
        if x == group_sz {
            return acc + 1
        }
        acc
    });
    // qa.clear();
    // group_sz = 0;

    println!("{}", result);

    Ok(())
}

