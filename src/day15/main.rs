#![feature(iterator_try_collect)]
use std::env;
use std::fs::File;
use std::io::prelude::*;

use std::io::ErrorKind;
type IoError = std::io::Error;

type Point = (i32, i32);
type Interval = (i32, i32);

fn mdist(p: Point, q: Point) -> i32 {
    (p.0 - q.0).abs() + (p.1 - q.1).abs()
}

// requires fst < snd
fn merge(fst: Interval, snd: Interval) -> Option<Interval> {
    if snd.0 <= fst.1 {
        return Some((std::cmp::min(fst.0, snd.0), std::cmp::max(fst.1, snd.1)));
    }
    None
}

#[derive(Debug)]
struct Cover {
    signal: Point,
    mrange: i32,
}

impl Cover {
    fn new(signal: Point, beacon: Point) -> Self {
        Self { signal, mrange: mdist(signal, beacon)}
    }
}

fn parse_assigned(curr: &str) -> std::io::Result<i32> {
    let start: usize;
    let sign: i32;
    if curr.chars().nth(0).eq(&Some('-')) {
        start = 1;
        sign = -1;
    } else {
        start = 0;
        sign = 1;
    }
    let off = curr[start..].chars().take_while(|c| c.is_digit(10)).count();
    curr[start..start + off].parse::<i32>()
                            .map_err(|e| IoError::new(ErrorKind::InvalidInput, e.to_string()))
                            .map(|res| sign * res)
}

fn parse_input(file: &File) -> std::io::Result<Vec<Cover>> {
    let reader = std::io::BufReader::new(file);
    let covers: Vec<Cover> = reader.lines().map(|line| -> std::io::Result<_> {
        let lstr = line?;
        let mut curr = &lstr[lstr.find("x=").unwrap() + 2..];
        let sx = parse_assigned(curr)?;
        curr = &curr[curr.find("y=").unwrap() + 2..];
        let sy = parse_assigned(curr)?;
        curr = &curr[curr.find("x=").unwrap() + 2..];
        let bx = parse_assigned(curr)?;
        curr = &curr[curr.find("y=").unwrap() + 2..];
        let by = parse_assigned(curr)?;
        Ok(Cover::new((sx, sy), (bx, by)))
    }).try_collect()?;
    Ok(covers)
}

fn row_cover(y_target: i32, covers: &Vec<Cover>) -> Vec<Interval> {
    let mut ivals: Vec<Interval> = covers.iter()
        .filter(|&cv| (cv.signal.1 - y_target).abs() <= cv.mrange)
        .map(|cv| {
            let delta = cv.mrange - (cv.signal.1 - y_target).abs();
            let s = cv.signal.0 - delta;
            let e = cv.signal.0 + delta;
            (s, e) 
        }).collect();
    
    ivals.sort_by(|(x0b, x0e), (x1b, x1e)| {
        let o = x0b.partial_cmp(x1b).unwrap();
        if o == std::cmp::Ordering::Equal {
            return x0e.partial_cmp(x1e).unwrap();
        }
        o
    });

    // merge overlapping intervals
    for _ in 0..ivals.len() {
        if ivals.len() == 1 {
            break;
        }
        let mut merged = Vec::new();
        while ivals.len() > 1 {
            let snd = ivals.pop().unwrap();
            let fst = ivals.pop().unwrap();
            if let Some(thd) = merge(fst, snd) {
                merged.push(thd);
            } else {
                merged.push(fst);
                merged.push(snd);
            }
        }
        if ivals.len() > 0 {
            merged.push(ivals.pop().unwrap());
        }
        ivals = merged;
    }
    ivals
}

pub fn solve2(file: &File) -> std::io::Result<()> {
    let covers = parse_input(file)?;

    let mut with_hole: Vec<Interval> = Vec::new();
    let mut ycoord = 0;
    for y in 0..4000000 {
        let ivals = row_cover(y as i32, &covers);
        if ivals.len() > 1 {
            println!("y = {} --> {:?}", y, ivals);
            with_hole = ivals;
            ycoord = y;
        }
    }
    let xcoord = with_hole[0].1 + 1;
    println!("x = {} y = {}", xcoord, ycoord);
    println!("solution 2: {}", (xcoord as usize) * 4000000 + (ycoord as usize));
    Ok(())
}

pub fn solve1(file: &File) -> std::io::Result<()> {
    let covers = parse_input(file)?;
    let y_target = 2000000;
    let ivals = row_cover(y_target, &covers);
    let d = (ivals[0].1 - ivals[0].0).abs();
    println!("solution 1: {}", d);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        return Err(IoError::new(ErrorKind::InvalidInput, "Expected filename as argument"));
    }
    let file = File::open(&argv[1])?;
    // solve1(&file)?;
    solve2(&file)?;

    Ok(())
}
