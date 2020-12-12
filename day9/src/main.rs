use std::fs::File;
use std::io::{self, prelude::*};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    // return part1();
    return part2();
}

// starts with 25 number preamble
// each number after should be the sum of any 2 of the 25 prev numbers
fn part1() -> io::Result<()> {
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    let nums: Vec<u64> = contents.split("\n").map(|x| {x.trim().parse::<u64>().unwrap()}).collect();

    fn contains_2sum(prev: Vec<u64>, target: u64) -> bool {
        let mut rem: HashSet<u64> = HashSet::new();
        for n in prev.iter() {
            if rem.contains(n) {
                return true;
            } else if n < &target {
                // println!("{} {} {}", target, n, target - n);
                rem.insert(target - n);
            }
        }
        false
    }
    
    let mut idx = 25;
    for n in nums[25..].iter() {
        // search prev 25 for 2 sum
        // println!("{} {:?}", idx, nums[idx-25..idx].to_vec());
        if contains_2sum(nums[idx-25..idx].to_vec(), *n) {
            idx += 1;
        } else {
            println!("result: {}", n);
            break;
        }
    }

    Ok(())
}

// find the contiguous range that sums to 21806024
fn part2() -> io::Result<()> {
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    let nums: Vec<u64> = contents.split("\n").map(|x| {x.trim().parse::<u64>().unwrap()}).collect();
    
    let target = 21806024;
    let mut s = 0;
    let mut e = 1;
    let mut sum = nums[s] + nums[e];
    while sum != target {
        println!("{} {} {}", s, e, sum);
        if sum < target {
            e += 1;
            sum += nums[e];
        } else if sum > target {
            sum -= nums[s];
            s += 1;
        }
    }
    println!("{} {} {}", s, e, sum);

    // find largest and smallest in contiguous range
    let range = nums[s..e+1].to_vec();
    let smallest = range.iter().min().unwrap();
    let largest = range.iter().max().unwrap();
    // println!("{} {} {:?}", nums[s], nums[e], nums[s..e+1].to_vec());
    println!("result: {} ", smallest + largest);

    Ok(())
}
