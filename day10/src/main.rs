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
    let mut nums: Vec<u64> = contents.split("\n").map(|x| {x.trim().parse::<u64>().unwrap()}).collect();
    nums.sort();
    nums.insert(0, 0);
    nums.push(nums.last().unwrap() + 3);
    // println!("{:?}", nums);

    let mut one_diff = 0;
    let mut three_diff = 0;
    for i in 1..nums.len() {
        let rem = nums[i] - nums[i-1];
        if rem == 1 {
            one_diff += 1;
        } else if rem == 3 {
            three_diff += 1;
        }
    }
    println!("{}", one_diff * three_diff);

    Ok(())
}

fn part2() -> io::Result<()> {
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    let mut nums: Vec<u64> = contents.split("\n").map(|x| {x.trim().parse::<u64>().unwrap()}).collect();
    nums.sort();
    nums.insert(0, 0);
    nums.push(nums.last().unwrap() + 3);
    println!("{:?}", nums);
    println!("---");

    // 1. find groupings surrounded by sentinels (non-removable adapters)
    let mut groups: Vec<Vec<u64>> = Vec::new();
    let mut cur_group: Vec<u64> = Vec::new();
    for i in 0..nums.len() {

        // sentinel check
        let mut is_sentinel = false;
        if i == nums.len()-1 || i == 0 {
            is_sentinel = true;
        } else {
            let prev_diff = nums[i] - nums[i-1];
            let next_diff = nums[i+1] - nums[i];
            if prev_diff == 3 || next_diff == 3 {
                is_sentinel = true;
            }
        }

        println!("{} {}", i, is_sentinel);
        if is_sentinel && cur_group.len() == 0 {
            // start sentinel
            cur_group.push(nums[i]);
        } else if is_sentinel && cur_group.len() == 1 {
            // replace existing sentinel with new start sentinel
            cur_group.clear();
            cur_group.push(nums[i]);
        } else if is_sentinel && cur_group.len() > 1 {
            // end of group
            cur_group.push(nums[i]);
            // println!("{:?}", cur_group);
            groups.push(cur_group.clone());
            cur_group.clear();
        } else if !is_sentinel && cur_group.len() >= 1 {
            // start sentinel exists, can fill with removable values
            cur_group.push(nums[i]);
        }
    }
    // 2. transform groups into patterns
    // a pattern contains only the diffs between the values
    let mut patterns: Vec<Vec<u64>> = Vec::new();
    for group in groups {
        let mut pattern: Vec<u64> = Vec::new();
        for i in 0..group.len()-1 {
            pattern.push(group[i+1] - group[i]);
        }
        patterns.push(pattern.clone());
        println!("{:?}", group);
        // println!("{:?}", pattern);
    }
    println!("---");

    // 3. for each pattern, find number of permutations
    // [3] [2] [1] -> 1
    // [3, 1] -> 1
    // [1, 1, 1] -> 4
    // [2, 1] [1, 1] -> 2
    // could add memoization for patterns but already fast enough
    fn count_perms(pattern: Vec<u64>, mut unique_perms: HashSet<Vec<u64>>) -> HashSet<Vec<u64>> {
        if pattern.len() == 1 {
            unique_perms.insert(pattern);
            return unique_perms;
        }
        // try to combine with next neighbor
        for i in 0..pattern.len()-1 {
            if pattern[i] + pattern[i+1] <= 3 {
                // combine with next neighbor
                let mut new_pattern = pattern.clone();
                new_pattern[i+1] = new_pattern[i] + new_pattern[i+1];
                new_pattern.remove(i);

                unique_perms.insert(new_pattern.clone());
                let new_patterns = count_perms(new_pattern, unique_perms.clone());
                unique_perms.extend(new_patterns);
            }
        }
        return unique_perms;
    }

    // let mut perms: Vec<Vec<i32>> = Vec::new();
    let mut result = 1;
    for pattern in patterns {
        // find number of permissable patterns, memoizable
        let mut init_perm = HashSet::new();
        init_perm.insert(pattern.clone());
        let perms = count_perms(pattern, init_perm);
        println!("{} {:?}", perms.len(), perms);
        result *= perms.len();
    }

    println!("{}", result);

    Ok(())
}
