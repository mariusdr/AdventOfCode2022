use std::env;
use std::fs::File;
use std::io::prelude::*;

fn sliding_windows<'a>(inp: &'a str, len: usize) -> impl Iterator<Item = (usize, &'a str)> {
    inp.char_indices().flat_map(move |(from, _)| {
        let start = &inp[from ..];
        start.char_indices().skip(len - 1).next().map(|(to, _)| {
            (from, &inp[from .. from + to + 1])
        }) 
    })
}

fn is_marking(slice: &[u8]) -> bool {
    for (i, x) in slice.iter().enumerate() {
        for (j, y) in slice.iter().enumerate() {
            if i != j && x == y {
                return false;
            }
        }
    }
    true
}

fn challenge1(file: File) -> std::io::Result<()> {
    let reader = std::io::BufReader::new(&file);        
    for line in reader.lines() {
        for (start, window) in sliding_windows(&line?, 4) {
            if is_marking(window.as_bytes()) {
                println!("mark {} starts at {}", window, start);
                println!("chars processed {}", start + 4);
                break;
            }
        }
    } 
    Ok(())
}

fn challenge2(file: File) -> std::io::Result<()> {
    let reader = std::io::BufReader::new(&file);        
    for line in reader.lines() {
        for (start, window) in sliding_windows(&line?, 14) {
            if is_marking(window.as_bytes()) {
                println!("mark {} starts at {}", window, start);
                println!("chars processed {}", start + 14);
                break;
            }
        }
    } 
    Ok(())
}

fn main() -> std::io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Expected filename as argument"));
    }
    let file = File::open(&argv[1])?;
    challenge1(file)?;
    challenge2(file)?;
    Ok(())
}
