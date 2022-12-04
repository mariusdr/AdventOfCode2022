use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;

fn main() -> std::io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Expected filename as argument"));
    }
    let file = File::open(&argv[1])?;
    let reader = std::io::BufReader::new(file);    
    
    let mut buckets: Vec<(usize, u32)> = Vec::new();
    let mut sum: u32 = 0;
    for line in reader.lines() {
        let lstr = line?;
        if lstr.len() == 0 {
            buckets.push((buckets.len(), sum));
            sum = 0;
        } else {
            sum += lstr.parse::<u32>().unwrap();
        }
    }

    buckets.sort_by(|(_, xs), (_, ys)| -> Ordering {
        if ys < xs {
            return Ordering::Less;
        } 
        if ys > xs {
            return Ordering::Greater;
        }
        Ordering::Equal
    });

    println!("solution 1 {:?}", buckets[0]);
    let (_, x0) = buckets[0];
    let (_, x1) = buckets[1];
    let (_, x2) = buckets[2];
    println!("solution 2: {}", x0 + x1 + x2);
    Ok(())
}