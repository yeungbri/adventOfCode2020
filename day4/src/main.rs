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

    let mut result = 0;
        
    // macro_rules! vec_of_strings {
    //     ($($x:expr),*) => (vec![$($x.to_string()),*]);
    // }

    // check each passport has req_fields
    let req_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut fields: HashMap<String, bool> = HashMap::new();
    for field in req_fields.iter() {
        fields.insert(field.to_string(), false);
    }

    // passports are delimited by blank lines
    for line in reader.lines() {
        // println!("{}", line?);
        let line: String = match line {
            Ok(s) => s,
            Err(_e) => "".to_string()
        };

        if line.trim().is_empty() {
            // new passport: record the result, reset the bookkeeping
            let valid = fields.values().all(|&x| x == true);
            if valid {
                // println!("correct");
                result += 1;
            }

            // reset the hashmap
            // println!("---");
            for field in req_fields.iter() {
                fields.insert(field.to_string(), false);
            }
        } else {
            // parse the line and record present fields
            let pairs: Vec<&str> = line.split(" ").collect();

            for pair in pairs {
                let kv: Vec<&str> = pair.split(":").collect();
                let key = kv[0];
                if req_fields.contains(&key) {
                    // println!("{}", key);
                    fields.insert(key.to_string(), true);
                }
            }
        }
    }

    let valid = fields.values().all(|&x| x == true);
    if valid {
        result += 1;
    }

    // println!("{} {}", width, height);
    println!("{}", result);

    Ok(())
}

fn part2() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;
        
    // macro_rules! vec_of_strings {
    //     ($($x:expr),*) => (vec![$($x.to_string()),*]);
    // }

    // check each passport has req_fields
    let req_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut fields: HashMap<String, bool> = HashMap::new();
    for field in req_fields.iter() {
        fields.insert(field.to_string(), false);
    }

    // passports are delimited by blank lines
    for line in reader.lines() {
        // println!("{}", line?);
        let line: String = match line {
            Ok(s) => s,
            Err(_e) => "".to_string()
        };

        if line.trim().is_empty() {
            // new passport: record the result, reset the bookkeeping
            let valid = fields.values().all(|&x| x == true);
            if valid {
                // println!("correct");
                result += 1;
            }

            // reset the hashmap
            // println!("---");
            for field in req_fields.iter() {
                fields.insert(field.to_string(), false);
            }
        } else {
            // parse the line and record present fields
            let pairs: Vec<&str> = line.split(" ").collect();

            for pair in pairs {
                let kv: Vec<&str> = pair.split(":").collect();
                let key = kv[0];
                let val = kv[1];

                if req_fields.contains(&key) {
                    let is_valid = match key {
                        "byr" => {
                            let int_val = val.parse::<i32>().unwrap_or_default();
                            int_val >= 1920 && int_val <= 2002
                        },
                        "iyr" => {
                            let int_val = val.parse::<i32>().unwrap_or_default();
                            int_val >= 2010 && int_val <= 2020 
                        },
                        "eyr" => {
                            let int_val = val.parse::<i32>().unwrap_or_default();
                            int_val >= 2020 && int_val <= 2030
                        },
                        "hgt" => {
                            // let last_char: char = val.chars().last().unwrap();
                            // let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
                            let re = Regex::new(r"^([0-9]*)([a-z]*)$").unwrap();
                            let caps = re.captures(val).unwrap();
                            let num = caps.get(1).map_or("", |m| m.as_str());
                            let num = num.parse::<i32>().unwrap_or_default();
                            let units = caps.get(2).map_or("", |m| m.as_str());

                            match units {
                                "in" => num >= 59 && num <= 76,
                                "cm" => num >= 150 && num <= 193,
                                _ => false
                            }
                        },
                        "hcl" => {
                            let re = Regex::new("^#[0-9a-f]{6}$").unwrap();
                            re.is_match(val)
                        },
                        "ecl" => {
                            let valid_ecls = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                            valid_ecls.contains(&val)
                        },
                        "pid" => {
                            let re = Regex::new("^[0-9]{9}$").unwrap();
                            re.is_match(val)
                        },
                        _ => false
                    };

                    if is_valid {
                        // println!("{}", key);
                        fields.insert(key.to_string(), true);
                    }
                }
            }
        }
    }

    let valid = fields.values().all(|&x| x == true);
    if valid {
        result += 1;
    }

    // println!("{} {}", width, height);
    println!("{}", result);

    Ok(())
}