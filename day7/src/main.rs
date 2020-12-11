use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use regex::Regex;

fn main() -> io::Result<()> {
    // return part1();
    return part2();
}

fn part1() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // {'light red': [(1, 'bright white'), (2, "muted yellow")],
    // "faded blue": []}
    let mut rules: HashMap<String, Vec<(i32, String)>> = HashMap::new();

    // parse rules and record
    for line in reader.lines() {
        let line: String = match line {
            Ok(s) => s,
            Err(_e) => "".to_string()
        };

        // println!("{}", line?);
        let rule:Vec<&str> = line.split(" bags contain").collect();
        let outside = rule[0];
        let contents_str = rule[1];
        // println!("{} {}", outside, contents_str);

        let mut contents: Vec<(i32, String)> = Vec::new();
        for content in contents_str.split(",") {
            let re = Regex::new(r"([0-9]*) ([a-z ]*) bag").unwrap();
            let caps = re.captures(content).unwrap();
            let num = caps.get(1).unwrap().as_str();
            let num = num.parse::<i32>().unwrap_or_default();
            let key = caps.get(2).unwrap().as_str();

            if num != 0 {
                contents.push((num, key.to_string()));
            }
            // println!("{}, {}", num, key);
        }
        rules.insert(outside.to_string(), contents);
    }

    // run through each color and search for shiny gold
    let mut result = 0;
    let mut memo: HashMap<String, bool> = HashMap::new();

    fn contains_gold(rules: &HashMap<String, Vec<(i32, String)>>, 
        memo: &mut HashMap<String, bool>, key: String, is_outer: bool) -> bool {
        if key == "shiny gold" && !is_outer {
            return true
        } else if memo.contains_key(&key) {
            return *memo.get(&key).unwrap();
        }

        let mut result: bool = false;
        // println!("{},{}", key, key.chars().count());
        let val: &Vec<(i32, String)> = rules.get(&key).unwrap();
        for t in val.iter() {
            let (_num, key) = t;
            result = result || contains_gold(rules, memo, key.to_string(), false);
            if result == true {
                break;
            }
        }
        memo.insert(key, result);
        result
    };

    // use memoization for bags that contain shiny gold
    for color in rules.keys() {
        if contains_gold(&rules, &mut memo, color.to_string(), true) {
            result += 1;
        }
    }
    println!("{}", result);

    Ok(())
}

fn part2() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // {'light red': [(1, 'bright white'), (2, "muted yellow")],
    // "faded blue": []}
    let mut rules: HashMap<String, Vec<(i32, String)>> = HashMap::new();

    // parse rules and record
    for line in reader.lines() {
        let line: String = match line {
            Ok(s) => s,
            Err(_e) => "".to_string()
        };

        // println!("{}", line?);
        let rule:Vec<&str> = line.split(" bags contain").collect();
        let outside = rule[0];
        let contents_str = rule[1];
        // println!("{} {}", outside, contents_str);

        let mut contents: Vec<(i32, String)> = Vec::new();
        for content in contents_str.split(",") {
            let re = Regex::new(r"([0-9]*) ([a-z ]*) bag").unwrap();
            let caps = re.captures(content).unwrap();
            let num = caps.get(1).unwrap().as_str();
            let num = num.parse::<i32>().unwrap_or_default();
            let key = caps.get(2).unwrap().as_str();

            if num != 0 {
                contents.push((num, key.to_string()));
            }
            // println!("{}, {}", num, key);
        }
        rules.insert(outside.to_string(), contents);
    }


    // counts the number of bags inside the colored bag
    let mut memo: HashMap<String, i32> = HashMap::new();
    fn count_inner(rules: &HashMap<String, Vec<(i32, String)>>, 
        memo: &mut HashMap<String, i32>, color: String, level: i32) -> i32 {
        if memo.contains_key(&color) {
            // println!("memo used!");
            return *memo.get(&color).unwrap();
        }

        let mut result = 0;
        let val: &Vec<(i32, String)> = rules.get(&color).unwrap();
        for t in val.iter() {
            let (num, key) = t;
            // println!("{}: {} {} = {}", level, num, key, result);
            result += num * count_inner(rules, memo, key.to_string(), level + 1) + num;
        }
        println!("--- {} {}", color, result);
        memo.insert(color, result);
        result
    }

    let result = count_inner(&rules, &mut memo, "shiny gold".to_string(), 1);
    println!("{}", result);

    Ok(())
}
