use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Interval {
    start: u32,
    end: u32,
}

impl Interval {
    pub fn new(start: u32, end: u32) -> Interval {
        Interval { start, end }
    }

    pub fn is_in(&self, outer: &Interval) -> bool {
        outer.start <= self.start && self.end <= outer.end
    }

    pub fn is_cut(&self, rhs: &Interval) -> bool {
        self.start <= rhs.start && rhs.start <= self.end
    }
}

fn parseinp(line: &str) -> std::io::Result<Vec<Interval>> {
    use std::io::{Error, ErrorKind};
    let mut vres: Vec<Interval> = Vec::new();
    for part in line.split(',') {
        let mut it = part.split('-');
        let leftnum = it.next().ok_or(Error::new(ErrorKind::InvalidInput, "parseinp"))?;
        let rightnum = it.next().ok_or(Error::new(ErrorKind::InvalidInput, "parseinp"))?;
        let ln = leftnum.parse::<u32>().map_err(|_| Error::new(ErrorKind::InvalidInput, "parsenum"))?;
        let rn = rightnum.parse::<u32>().map_err(|_| Error::new(ErrorKind::InvalidInput, "parsenum"))?;
        vres.push(Interval::new(ln, rn));
    }
    Ok(vres)
}

fn challenge1(file: File) -> std::io::Result<()> {
    let reader = std::io::BufReader::new(file);
    let mut hitcnt = 0u32;
    for line in reader.lines() {
        let xs = parseinp(&line?)?;
        if xs[0].is_in(&xs[1]) || xs[1].is_in(&xs[0]) {
            // println!("hit at {:?} {:?}", xs[0], xs[1]);
            hitcnt += 1;
        }
    }
    println!("solution1: {}", hitcnt);
    Ok(())
}

fn challenge2(file: File) -> std::io::Result<()> {
    let reader = std::io::BufReader::new(file);
    let mut hitcnt = 0u32;
    for line in reader.lines() {
        let xs = parseinp(&line?)?;
        if xs[0].is_cut(&xs[1]) || xs[1].is_cut(&xs[0]) {
            // println!("hit at {:?} {:?}", xs[0], xs[1]);
            hitcnt += 1;
        }
    }
    println!("solution2: {}", hitcnt);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Expected filename as argument"));
    }
    challenge1(File::open(&argv[1])?)?;
    challenge2(File::open(&argv[1])?)?;
 
    Ok(())
}
