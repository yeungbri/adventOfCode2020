(advent of code 2020)[https://adventofcode.com/2020]

Using this challenge to learn Rust!

# Rust setup
Rust is installed via Rustup, Rust's installer and version management tool  
Update Rust: `rustup update`  
Build project: `cargo build`
Run: `cargo run`  
Test: `cargo test`  
Docs: `cargo doc`  
Publish library to crates.io: `cargo publish`  

# Rust basics
cargo new project-name
Cargo.toml - manifest file for Rust, contains metadata and deps
find libraries on crates.io
```
[depedencies]
crate-name = "version"
```
cargo build to install deps
Cargo.lock - log of exact versions we are using locally
To use a dep:
`use crate_name::say;` to import say function from crate_name

