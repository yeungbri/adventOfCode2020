use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    // return part1();
    return part2();
}

fn check_adj<T>(r: usize, c: usize, grid: Vec<Vec<char>>, pred: T) -> bool where T: Fn(char, i32) -> (bool, i32) {
    let left_border = r == 0;
    let top_border = c == 0;
    let right_border = r == grid.len()-1;
    let bot_border = c == grid[0].len()-1;
    let mut count = 0;

    let mut run_pred = |seat| {
        let (should_exit, new_count) = pred(seat, count);
        count = new_count;
        should_exit
    };
    
    if !left_border {
        let left = grid[r-1][c];
        if run_pred(left) {
            return false;
        }

        if !top_border {
            let tl = grid[r-1][c-1];
            if run_pred(tl) {
                return false;
            }
        }

        if !bot_border {
            let bl = grid[r-1][c+1];
            if run_pred(bl) {
                return false;
            }
        }
    } 
    if !right_border {
        let right = grid[r+1][c];
        if run_pred(right) {
            return false;
        }

        if !top_border {
            let tr = grid[r+1][c-1];
            if run_pred(tr) {
                return false;
            }
        }

        if !bot_border {
            let br = grid[r+1][c+1];
            if run_pred(br) {
                return false;
            }
        }
    }
    if !top_border {
        let top = grid[r][c-1];
        if run_pred(top) {
            return false;
        }
    }
    if !bot_border {
        let bot = grid[r][c+1];
        if run_pred(bot) {
            return false;
        }
    }
    true
}

// slow
// opposite way: to update neighbors based on self (i.e. if occupied update: empties to false for switching, and occupied count + 1; at the end, loop through and swap if empties are still true and occupied count < threshold)
fn part1() -> io::Result<()> {
    let file = File::open("example.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        // println!("{}", line?);
        let row: Vec<char> = line?.chars().collect();
        grid.push(row);
    }

    // simulate
    fn adj_empty(seat: char, acc: i32) -> (bool, i32) {
        let occupied = '#';
        (seat == occupied, acc)
    }

    fn not_crowded(seat: char, acc: i32) -> (bool, i32) {
        let is_occupied = seat == '#';
        if is_occupied {
            return (acc >= 3, acc + 1);
        }
        (acc >= 4, acc)
    }

    let mut changed = true;
    // for x in 0..2 {
    while changed {
        changed = false;
        let mut new_grid: Vec<Vec<char>> = grid.clone();
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                let seat = grid[r][c];
                if seat == 'L' {
                    // no occupied adjacent -> occupied
                    let is_adj_empty = check_adj(r, c, grid.clone(), adj_empty); 
                    if is_adj_empty{
                        new_grid[r][c] = '#';
                        changed = true;
                    }
                } else if seat == '#' {
                    // 4+ seats adj are occupied -> empty
                    if !check_adj(r, c, grid.clone(), not_crowded) {
                        new_grid[r][c] = 'L';
                        changed = true;
                    }
                }
            }
        }
        grid = new_grid;
        // for row in &grid {
        //     println!("{:?}", row);
        // }
        // println!("");
    }

    // count occupied seats
    let mut result = 0;
    for row in grid {
        for seat in row {
            if seat == '#' {
                result += 1;
            }
        }
    }
    println!("{}", result);

    Ok(())
}

fn part2() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        // println!("{}", line?);
        let row: Vec<char> = line?.chars().collect();
        grid.push(row);
    }

    // generate mapping of visible seats for each seat
    fn find_visible(mut x: usize, mut y: usize, grid: Vec<Vec<char>>, dx: i32, dy: i32) -> Option<(usize, usize)> {
        
        let should_stop = |x, y, dx, dy| {
            x == 0 && dx < 0 
                || x == (grid.len()-1) && dx > 0
                || y == 0 && dy < 0 
                || y == (grid[0].len()-1) && dy > 0
        };

        while !should_stop(x, y, dx, dy) {
            // println!("{} {} {} {} {}", x, dx, y, dy, grid.len()-1);
            x = (x as i32 + dx) as usize;
            y = (y as i32 + dy) as usize;
            if grid[x][y] == 'L' || grid[x][y] == '#' {
                return Some((x, y));
            }
        }
        // println!("---");
        return None;
    }

    struct Neighbors {
        top: Option<(usize, usize)>,
        right: Option<(usize, usize)>,
        left: Option<(usize, usize)>,
        bot: Option<(usize, usize)>,
        tl: Option<(usize, usize)>,
        tr: Option<(usize, usize)>,
        bl: Option<(usize, usize)>,
        br: Option<(usize, usize)>
    }

    let mut visible: Vec<Vec<Neighbors>> = Vec::new();
    for x in 0..grid.len() {
        let mut row_mappings = Vec::new();
        for y in 0..grid[x].len() {
            let top = find_visible(x, y, grid.clone(), 0, 1);
            let bot = find_visible(x, y, grid.clone(), 0, -1);
            let left = find_visible(x, y, grid.clone(), -1, 0);
            let right = find_visible(x, y, grid.clone(), 1, 0);
            let tl = find_visible(x, y, grid.clone(), -1, 1);
            let tr = find_visible(x, y, grid.clone(), 1, 1);
            let br = find_visible(x, y, grid.clone(), 1, -1);
            let bl = find_visible(x, y, grid.clone(), -1, -1);
            
            let mapping = Neighbors {
                top: top,
                bot: bot,
                left: left,
                right: right,
                tl: tl,
                tr: tr,
                br: br,
                bl: bl
            };
            row_mappings.push(mapping);
        }
        visible.push(row_mappings);
    }

    // simulate
    fn check_visible<T>(r: usize, c: usize, grid: Vec<Vec<char>>, neighbors: &Neighbors, pred: T) -> bool where T: Fn(char, i32) -> (bool, i32) {
        let mut count = 0;
        // let neighbors = &visible[r][c];

        let mut run_pred = |seat| {
            let (should_exit, new_count) = pred(seat, count);
            count = new_count;
            should_exit
        };
        
        let coords = [
            neighbors.top,
            neighbors.bot,
            neighbors.left,
            neighbors.right,
            neighbors.tl,
            neighbors.tr,
            neighbors.br,
            neighbors.bl,
        ];

        for coord in coords.iter() {
            match coord {
                Some(coord) => {
                    if run_pred(grid[coord.0][coord.1]) {
                        return false;
                    }
                },
                None => ()
            }
        }
        true
    }

    fn adj_empty(seat: char, acc: i32) -> (bool, i32) {
        let occupied = '#';
        (seat == occupied, acc)
    }

    fn not_crowded(seat: char, acc: i32) -> (bool, i32) {
        let is_occupied = seat == '#';
        if is_occupied {
            return (acc >= 4, acc + 1);
        }
        (acc >= 5, acc)
    }

    let mut changed = true;
    // for x in 0..2 {
    while changed {
        changed = false;
        let mut new_grid: Vec<Vec<char>> = grid.clone();
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                let seat = grid[r][c];
                let neighbors = &visible[r][c];
                if seat == 'L' {
                    // no occupied adjacent -> occupied
                    let is_adj_empty = check_visible(r, c, grid.clone(), neighbors, adj_empty); 
                    if is_adj_empty{
                        new_grid[r][c] = '#';
                        changed = true;
                    }
                } else if seat == '#' {
                    // 4+ seats adj are occupied -> empty
                    if !check_visible(r, c, grid.clone(), neighbors, not_crowded) {
                        new_grid[r][c] = 'L';
                        changed = true;
                    }
                }
            }
        }
        grid = new_grid;
        // for row in &grid {
        //     println!("{:?}", row);
        // }
        // println!("");
    }

    // count occupied seats
    let mut result = 0;
    for row in grid {
        for seat in row {
            if seat == '#' {
                result += 1;
            }
        }
    }
    println!("{}", result);

    Ok(())
}
