use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod program;
use program::InterpretIter;

fn main() {
    let file_path = "../input.txt";
    //part 1
    {
        let path = Path::new(file_path);
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
    //part 2
    {
        let file = File::open(file_path).unwrap();
        let mut signals = io::BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .interpret();
        let dimensions = (40, 6);
        let sprite_len = 3;
        let (pad_left, pad_right) = {
            let sprite_half_len = sprite_len / 2;
            (sprite_half_len, sprite_len - 1 - sprite_half_len)
        };
        println!("part 2:");
        for _ in 0..dimensions.1 {
            let mut line = String::new();
            for i in 0..dimensions.0 {
                let sprite_position = signals.next().unwrap();
                let sprite_area = (sprite_position - pad_left)..=(sprite_position + pad_right);
                line.push(if sprite_area.contains(&i) { '#' } else { '.' });
                line.push(' ');
            }
            println!("{}", line);
        }
    }
}
