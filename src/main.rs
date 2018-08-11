use std::fs::File;
use std::io::{Write, BufReader, BufRead};

fn main() {
    let path = "lines.txt";

    let mut output = File::create(path).unwrap();
    write!(output, "Rust\nis\nfun").unwrap();

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }
}
