use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    // return part1();
    return part2();
}

fn part1() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut treemap: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        // println!("{}", line?);
        let char_vec: Vec<char> = line?.chars().collect();
        treemap.push(char_vec);
    }

    let width = treemap[0].iter().count();
    let height = treemap.iter().count();
    // println!("{} {}", width, height);

    let mut result = 0;
    let mut x = 0;
    let mut y = 0;
    while y < height {
        if treemap[y][x] == '#' {
            result += 1;
        }
        x = (x + 3) % width;
        y += 1;
    }

    println!("{}", result);

    Ok(())
}

fn part2() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut treemap: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        // println!("{}", line?);
        let char_vec: Vec<char> = line?.chars().collect();
        treemap.push(char_vec);
    }

    let width = treemap[0].iter().count();
    let height = treemap.iter().count();
    // println!("{} {}", width, height);

    let mut result:u32 = 0;
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    for slope in slopes.iter() {
        let mut trees = 0;
        let mut x = 0;
        let mut y = 0;
        let (dx, dy) = slope;

        println!("{} {} {} {}", x, y, dx, dy);
        while y < height {
            if treemap[y][x] == '#' {
                trees += 1;
            }
            x = (x + dx) % width;
            y += dy;
        }

        if result == 0 {
            result = trees;
        } else {
            result *= trees;
        }
        println!("{} {}", trees, result);
    }

    println!("{}", result);

    Ok(())
}
