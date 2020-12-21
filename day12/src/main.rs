extern crate tempdir;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use tempdir::TempDir;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    // part1();
    part2(file);
    Ok(())
}

fn part1() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dir: u32 = 0; // EAST
    for line in reader.lines() {
        let line: String = match line {
            Ok(s) => s,
            Err(_e) => "".to_string()
        };

        let cmd = line[..1].to_string();
        let val = line[1..].to_string().parse::<i32>().unwrap();
        // println!("{} {} {} {}", cmd, val, x, y);

        match cmd.as_str() {
            "N" => y += val,
            "S" => y -= val,
            "W" => x -= val,
            "E" => x += val,
            "F" => {
                // println!("{} {} {} {}", cmd, val, x, y);
                match dir {
                    0 => x += val, // EAST
                    90 => y -= val, // SOUTH
                    180 => x -= val, // WEST
                    270 => y += val, // NORTH
                    _ => ()
                }
            },
            "L" => {
                // println!("{} {} {}", cmd, val, dir);
                // % is remainder not MODULUS!
                let a = dir as i32 - val;
                let b = 360;
                let modulus = ((a % b) + b) % b;
                dir = modulus as u32;
            },
            "R" => {
                // println!("{} {} {}", cmd, val, dir);
                let a = dir as i32 + val;
                let b = 360;
                let modulus = ((a % b) + b) % b;
                dir = modulus as u32;
            },
            _ => ()
        }
    }

    let result = x.abs() + y.abs();
    println!("{}", result);

    Ok(())
}

fn part2(file: File) -> i32 {
    let reader = BufReader::new(file);

    // waypoint, relative to the ship
    let mut wx: i32 = 10;
    let mut wy: i32 = 1;
    // ship
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut cnt = 0;
    for line in reader.lines() {
        let line: String = match line {
            Ok(s) => s,
            Err(_e) => "".to_string()
        };
        // println!("{}", line);

        let cmd = line[..1].to_string();
        let val = line[1..].to_string().parse::<i32>().unwrap();
        // println!("{} {} {} {} {} {}", cmd, val, x, y, wx, wy);

        match cmd.as_str() {
            "N" => wy += val,
            "S" => wy -= val,
            "W" => wx -= val,
            "E" => wx += val,
            "F" => {
                // println!("{} {} {} {} {} {}", cmd, val, x, y, wx, wy);
                // println!("{} {} {} {} {} {}", cmd, val, x, y, wx * val, wy * val);
                let prev_x = x;
                let prev_y = y;
                x += wx * val;
                y += wy * val;
                // println!("{} {} {} {} {}", val, x, y, prev_x - x, prev_y - y);
            },
            "L" => {
                // counter clockwise
                // println!("{} {} {} {}", cmd, val, wx, wy);
                match val {
                    0 => (),
                    90 => {
                        rotate90(&mut wx, &mut wy);
                        rotate90(&mut wx, &mut wy);
                        rotate90(&mut wx, &mut wy);
                    }
                    180 => {
                        rotate90(&mut wx, &mut wy);
                        rotate90(&mut wx, &mut wy);
                    },
                    270 => {
                        rotate90(&mut wx, &mut wy);
                    },
                    _ => {
                        println!("unexpected input");
                    }
                }
                // println!("{} {} {}", val, wx, wy);
            },
            "R" => {
                // clockwise
                // println!("{} {} {} {}", cmd, val, wx, wy);
                match val {
                    0 => (),
                    90 => rotate90(&mut wx, &mut wy),
                    180 => {
                        rotate90(&mut wx, &mut wy);
                        rotate90(&mut wx, &mut wy);
                    },
                    270 => {
                        rotate90(&mut wx, &mut wy);
                        rotate90(&mut wx, &mut wy);
                        rotate90(&mut wx, &mut wy);
                    },
                    _ => {
                        println!("unexpected input");
                    }
                }
                // println!("{} {} {}", val, wx, wy);
            },
            _ => println!("unexpected input")
        }

        // println!("{} {} {} {} {} {} {}", cnt, cmd, val, x, y, wx, wy);
        cnt += 1;
    }

    let result = x.abs() + y.abs();
    println!("{}", result);

    result
}

fn rotate90 (x: &mut i32, y: &mut i32) {
    let copy_y = *y;
    *y = -*x;
    *x = copy_y;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_testfile(s: &str) -> io::Result<(std::path::PathBuf, TempDir)> {
        let dir = TempDir::new("my_directory_prefix")?;
        let file_path = dir.path().join("foo.txt");
        println!("{:?}", file_path);
    
        // let mut f = File::create("test.txt")?;
        let mut f = File::create(file_path.clone())?;
        f.write_all(s.as_bytes())?;
        f.sync_all()?;
        
        // let f = File::open(file_path)?;
        // let result = part2(f);
        // assert_eq!(result, 110);

        // dir.close()?;
        Ok((file_path, dir))
    }

    #[test]
    fn test_forward() {
        
    }

    #[test]
    fn test_rotate90_fn() {
        let mut dx = 10;
        let mut dy = 1;
        rotate90(&mut dx, &mut dy);
        assert_eq!(dx, 1);
        assert_eq!(dy, -10);

        dx = 10;
        dy = 1;
        rotate90(&mut dx, &mut dy);
        rotate90(&mut dx, &mut dy);
        assert_eq!(dx, -10);
        assert_eq!(dy, -1);

        dx = 10;
        dy = 1;
        rotate90(&mut dx, &mut dy);
        rotate90(&mut dx, &mut dy);
        rotate90(&mut dx, &mut dy);
        assert_eq!(dx, -1);
        assert_eq!(dy, 10);

        dx = 10;
        dy = 1;
        rotate90(&mut dx, &mut dy);
        rotate90(&mut dx, &mut dy);
        rotate90(&mut dx, &mut dy);
        rotate90(&mut dx, &mut dy);
        assert_eq!(dx, 10);
        assert_eq!(dy, 1);
    }

    #[test]
    fn test_rotates() -> io::Result<()> {
        // let dir = TempDir::new("my_directory_prefix")?;
        // let file_path = dir.path().join("foo.txt");
        // println!("{:?}", file_path);
    
        // // let mut f = File::create("test.txt")?;
        // let mut f = File::create(file_path.clone())?;
        // f.write_all(b"R180\nF10")?;
        // f.sync_all()?;
        let (file_path, dir) = setup_testfile("R90\nF10").unwrap();
        let f = File::open(file_path).unwrap();
        let r90 = part2(f);
        assert_eq!(r90, 110);
        dir.close()?;

        let (file_path, dir) = setup_testfile("R180\nF10").unwrap();
        let f = File::open(file_path).unwrap();
        let r180 = part2(f);
        assert_eq!(r180, 110);
        dir.close()?;

        let (file_path, dir) = setup_testfile("R270\nF10").unwrap();
        let f = File::open(file_path).unwrap();
        let r270 = part2(f);
        assert_eq!(r270, 110);
        dir.close()?;

        let (file_path, dir) = setup_testfile("L90\nF10").unwrap();
        let f = File::open(file_path).unwrap();
        let l90 = part2(f);
        assert_eq!(l90, 110);
        dir.close()?;

        let (file_path, dir) = setup_testfile("L180\nF10").unwrap();
        let f = File::open(file_path).unwrap();
        let l180 = part2(f);
        assert_eq!(l180, 110);
        dir.close()?;

        let (file_path, dir) = setup_testfile("L270\nF10").unwrap();
        let f = File::open(file_path).unwrap();
        let l270 = part2(f);
        assert_eq!(l270, 110);
        dir.close()?;

        assert_eq!(l270, r90);
        assert_eq!(r270, l90);
        assert_eq!(l180, r180);
        Ok(())
    }
}