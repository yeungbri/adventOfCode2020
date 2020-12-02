use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
// use std::env;

fn read<R: Read>(io: R) -> Result<Vec<i32>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> Result<(), Error> {
    // let path = env::current_dir()?;
    // println!("The current directory is {}", path.display());
    // Ok(())

    let vec = read(File::open("input.txt")?)?;
    // println!("{}", part1(vec, 2020));
    println!("{}", part2(vec));
    Ok(())
}

fn part1(mut vec: Vec<i32>, target: i32) -> i32 {

    for x in &vec {
        let find = target - x;
        
        // print!("{} ", x)
    }
    return -1;
}

fn part2(mut vec: Vec<i32>) -> i32 {
    for (x_idx, x) in vec.iter().enumerate() {
        let target1 = 2020 - x;
        // println!("{} {}", x, target1);
        if target1 < 0 {
            continue
        }
        for (y_idx, y) in vec.iter().enumerate() {
            if y_idx != x_idx {
                // println!("{}, {}", x + y, target);
                let target2 = target1 - y;
                if target2 < 0 {
                    continue
                }
                for (z_idx, z) in vec.iter().enumerate() {
                    // println!("{}, {}, {}", x, y, z);
                    if z_idx != x_idx && z_idx != y_idx {
                        if target2 == *z {
                            return x * y * z;
                        }
                    }
                }
            }
        }
    }
    return -1;
}
