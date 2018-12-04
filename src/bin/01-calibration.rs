use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

/// This is for the 2nd version of day 1. The first was a simple `.fold()`.
fn main() -> io::Result<()> {
    let fp = env::args().skip(1).next().expect("file path");
    let f = File::open(fp)?;
    let reader = BufReader::new(f);
    let mut acc: i32 = 0;
    let mut seen = HashSet::new();
    let inputs: Vec<i32> = reader.lines().map(|x| x.unwrap().parse().unwrap()).collect();
    
    for value in inputs.iter().cycle() {
        acc += value;
        if seen.contains(&acc) {
            println!("{}", acc);
            return Ok(())
        }
        seen.insert(acc);
    }
    Ok(())
}
