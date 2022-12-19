use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod program;
use program::InterpretIter;

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(path).unwrap();
    let signals: Vec<(isize, isize)> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .interpret()
        .zip(1..)
        .skip(20 - 1)
        .step_by(40)
        .take(6)
        .collect();
    println!("signals: {:?}", signals);
    let sum: isize = signals.iter().map(|(value, cycle)| cycle * value).sum();
    println!("part 1: {}", sum);
}
