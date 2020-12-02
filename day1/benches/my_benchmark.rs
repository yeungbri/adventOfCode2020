use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::convert::TryInto;
// use std::env;

fn read<R: Read>(io: R) -> Result<Vec<i32>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn part1(mut vec: Vec<i32>) -> i32 {
    vec.sort();
    for x in &vec {
        let target = 2020 - x;
        // match vec.iter().position(|&n| n == target) {
        //     None => continue,
        //     Some(i) => return x * target,
        // }
        match vec.binary_search(&target) {
            Ok(v) => return x * target,
            Err(e) => continue,
        }
        // print!("{} ", x)
    }
    return -1;
}

fn demo<T>(v: Vec<T>) -> [T; 200] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", 200, v.len()))
}

// fn read_file() -> Result<Vec<i32>, Error> {
//     return read(File::open("input.txt")?)?;
// }

fn criterion_benchmark(c: &mut Criterion) {
    let vec = demo(read(File::open("input.txt").expect("failed to open")).expect("failed to parse"));
    c.bench_function(
        "part1",
        |b| b.iter(|| {
            part1(vec.to_vec())
        }),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);