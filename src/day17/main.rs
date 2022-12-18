#![feature(iterator_try_collect)]
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::PrefixComponent;

type IoError = std::io::Error;
type Point = (i64, i64);

struct PointTransform {
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
}

impl PointTransform {
    fn new(xmin: i64, xmax: i64, ymin: i64, ymax: i64) -> Self {
        Self { xmin, xmax, ymin, ymax }
    }

    fn bound_check(&self, p: Point) -> Option<Point> {
        let (x, y) = p;
        if x < self.xmin || y < self.ymin || y >= self.ymax {
            return None;
        } 
        Some(p)
    }

    fn apply(&self, p: Point, dx: i64, dy: i64) -> Option<Point> {
        let q = (p.0 + dx, p.1 + dy);
        self.bound_check(q)
    }
}

struct RockGen<'a> {
    pt: &'a PointTransform,
    next_pattern: u8,
}

impl<'a> RockGen<'a> {
    fn new(pt: &'a PointTransform) -> Self {
        Self { pt, next_pattern: 0 }
    }

    fn make_pattern(&mut self, x_max: i64) -> Option<Vec<Point>> {
        if self.next_pattern == 0 {
            let anchor = (x_max, 2);
            let patt = self.make_pattern_0(anchor);
            self.next_pattern = (self.next_pattern + 1) % 5;
            return patt;
        }
        if self.next_pattern == 1 {
            let anchor = (x_max, 2);
            let patt = self.make_pattern_1(anchor);
            self.next_pattern = (self.next_pattern + 1) % 5;
            return patt;
        }
        if self.next_pattern == 2 {
            let anchor = (x_max, 2);
            let patt = self.make_pattern_2(anchor);
            self.next_pattern = (self.next_pattern + 1) % 5;
            return patt;
        }
        if self.next_pattern == 3 {
            let anchor = (x_max, 2);
            let patt = self.make_pattern_3(anchor);
            self.next_pattern = (self.next_pattern + 1) % 5;
            return patt;
        }
        if self.next_pattern == 4 {
            let anchor = (x_max, 2);
            let patt = self.make_pattern_4(anchor);
            self.next_pattern = (self.next_pattern + 1) % 5;
            return patt;
        }
        None
    }

    // ....
    // ....
    // ....
    // A###
    fn make_pattern_0(&self, anchor: Point) -> Option<Vec<Point>> {
        let mut ps = Vec::new(); 
        ps.push(self.pt.apply(anchor, 0, 0)?);
        ps.push(self.pt.apply(anchor, 0, 1)?);
        ps.push(self.pt.apply(anchor, 0, 2)?);
        ps.push(self.pt.apply(anchor, 0, 3)?);
        Some(ps)
    }
    
    // ....
    // .#..
    // ###.
    // A#..
    fn make_pattern_1(&self, anchor: Point) -> Option<Vec<Point>> {
        let mut ps = Vec::new(); 
        ps.push(self.pt.apply(anchor, 2, 1)?);
        ps.push(self.pt.apply(anchor, 1, 0)?);
        ps.push(self.pt.apply(anchor, 1, 1)?);
        ps.push(self.pt.apply(anchor, 1, 2)?);
        ps.push(self.pt.apply(anchor, 0, 1)?);
        Some(ps)
    }

    // ....
    // ..#.
    // ..#.
    // A##.
    fn make_pattern_2(&self, anchor: Point) -> Option<Vec<Point>> {
        let mut ps = Vec::new(); 
        ps.push(self.pt.apply(anchor, 2, 2)?);
        ps.push(self.pt.apply(anchor, 1, 2)?);
        ps.push(self.pt.apply(anchor, 0, 2)?);
        ps.push(self.pt.apply(anchor, 0, 1)?);
        ps.push(self.pt.apply(anchor, 0, 0)?);
        Some(ps)
    }
    
    // #...
    // #...
    // #...
    // A...
    fn make_pattern_3(&self, anchor: Point) -> Option<Vec<Point>> {
        let mut ps = Vec::new(); 
        ps.push(self.pt.bound_check(anchor)?);
        ps.push(self.pt.apply(anchor, 1, 0)?);
        ps.push(self.pt.apply(anchor, 2, 0)?);
        ps.push(self.pt.apply(anchor, 3, 0)?);
        Some(ps)
    }
   
    // ....
    // ....
    // ##..
    // A#..
    fn make_pattern_4(&self, anchor: Point) -> Option<Vec<Point>> {
        let mut ps = Vec::new(); 
        ps.push(self.pt.apply(anchor, 1, 0)?);
        ps.push(self.pt.apply(anchor, 1, 1)?);
        ps.push(self.pt.apply(anchor, 0, 0)?);
        ps.push(self.pt.apply(anchor, 0, 1)?);
        Some(ps)
    }
}

struct RockStopper {
    first: HashSet<Point>,
    second: HashSet<Point>,
    third: HashSet<Point>,
    max_x: i64,
    active: u8,
}

// const INIT_CAP: usize = 5000000;
const INIT_CAP: usize = 25000000;
impl RockStopper {
    fn new() -> Self {
        Self { 
            first: HashSet::with_capacity(INIT_CAP), 
            second: HashSet::with_capacity(INIT_CAP), 
            third: HashSet::with_capacity(INIT_CAP), 
            max_x: 0,
            active: 0,
        }
    }

    fn prev(&self) -> &HashSet<Point> {
        if self.active == 0 {
            return &self.third;
        }
        if self.active == 1 {
            return &self.first;
        }
        &self.second
    }

    fn active(&self) -> &HashSet<Point> {
        if self.active == 0 {
            return &self.first;
        }
        if self.active == 1 {
            return &self.second;
        }
        &self.third
    }

    fn active_mut(&mut self) -> &mut HashSet<Point> {
        if self.active == 0 {
            return &mut self.first;
        }
        if self.active == 1 {
            return &mut self.second;
        }
        &mut self.third
    }

    fn insert_rock(&mut self, rock: &Vec<Point>) {
        if self.active().len() >= INIT_CAP {
            println!("fill factor {}", self.active().len());
            if self.active == 0 {
                self.second.drain();
            }
            if self.active == 1 {
                self.third.drain();
            }
            if self.active == 2 {
                self.first.drain();
            }
            self.active = (self.active + 1) % 3;
        }

        for p in rock {
            if self.active_mut().insert(*p) {
                self.max_x = std::cmp::max(self.max_x, p.0);
            }
        }
    }

    fn is_stopped(&self, rock: &Vec<Point>) -> bool {
        for p in rock {
            if self.active().contains(p) {
                return true;
            } else if self.prev().contains(p) {
                return true;
            } else if p.0 < 0 {
                return true;
            }
        }
        false
    }
}


fn transform(rock: &Vec<Point>, pt: &PointTransform, dx: i64, dy: i64) -> Option<Vec<Point>> {
    let mut rockt = rock.clone();
    for i in 0..rockt.len() {
        rockt[i] = pt.apply(rockt[i], dx, dy)?;
    }
    Some(rockt)
}

fn simulate(directions: &Vec<Direction>) {
    let mut stopped_rocks = RockStopper::new();
    let mut max_x = 0;
    let pt = PointTransform::new(0, 999999999, 0, 7);
    let mut rg = RockGen::new(&pt);
    let mut dircyc = directions.iter().cycle();
 
    for round in 0..1000000000000u64 {
    // for round in 0..2022 {
        let mut rock = rg.make_pattern(max_x + 3).unwrap();
        loop {

            let d = dircyc.next().unwrap();
            let dvec = match *d {
                Direction::Left => (0, -1),
                Direction::Right => (0, 1),
                _ => panic!("unexpected direction!")
            };
            if let Some(rockt) = transform(&rock, &pt, dvec.0, dvec.1) {
                if !stopped_rocks.is_stopped(&rockt) {
                    rock = rockt;
                }
            }

            if let Some(rockt) = transform(&rock, &pt, -1, 0) {
                if stopped_rocks.is_stopped(&rockt) {
                    break;
                }
                rock = rockt;
            } else {
                break;
            }
        }
        stopped_rocks.insert_rock(&rock);
        max_x = stopped_rocks.max_x + 1;

        if round % 100000 == 0 {
            println!("{} | new max_x {}", round, max_x);
            println!("{} rounds remaining", 1000000000000u64 - round);
        }
    }
    println!("max at end {}", max_x);
}


#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Down,
}

pub fn parse_inp(file: &File) -> std::io::Result<Vec<Direction>> {
    std::io::BufReader::new(file).bytes().map(|b| {
        let b = b.map_err(|e| IoError::new(ErrorKind::InvalidInput, e.to_string()))?;
        if b == '<' as u8 {
            return Ok(Direction::Left);
        } else if b == '>' as u8 {
            return Ok(Direction::Right);
        } 
        Err(IoError::new(ErrorKind::InvalidInput, "invalid sign"))
    }).try_collect()
}


fn main() -> std::io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        return Err(IoError::new(ErrorKind::InvalidInput, "Expected filename as argument"));
    }
    let file = File::open(&argv[1])?;
    let ds = parse_inp(&file)?;
    
    simulate(&ds);

    Ok(())
}
