use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, BufReader};

#[derive(Debug)]
struct Count {
    bytes: u64,
    chars: u64,
    lines: u64,
    words: u64,
}

#[derive(Parser, Debug)]
struct Args {
    /// Print the byte counts
    #[arg(short = 'c', long)]
    bytes: bool,

    /// Filename
    filename: String,
}

fn count<R: Read>(reader: BufReader<R>) -> Count {
    let mut res: Count = Count {
        bytes: 0,
        chars: 0,
        lines: 0,
        words: 0,
    };

    for line in reader.lines() {
        let line: String = line.ok().expect("IO error.");

        res.lines += 1;

        res.bytes += line.bytes().count() as u64;
        res.bytes += 2; // because \n

        res.words += line.split_whitespace().count() as u64;

        let length: u64 = line.chars().count() as u64;
        res.chars += length;
        res.chars += 1; // +1 because \n
    }

    res
}

fn main() {
    let args = Args::parse();

    dbg!(&args);

    let file = File::open(&args.filename)
        .ok()
        .expect("I couldn't open that file, sorry :(");
    dbg!("{} {}", count(BufReader::new(file)), args.filename);
}
