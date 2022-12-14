use std::env;
use std::fmt::Write;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

use std::io::ErrorKind;
type IoError = std::io::Error;

type Point = (u32, u32);

#[derive(Debug)]
struct LaneMap {
    lanes: HashMap<u32, HashSet<u32>>, 
    y_max: u32,
}

impl LaneMap {
    fn new() -> Self {
        Self { lanes: HashMap::new(), y_max: 0 }
    }

    fn insert(&mut self, p: Point, set_floor: bool) -> bool {
        let (x, y) = p;
        let s = self.lanes.entry(x).or_insert(HashSet::new());
        if set_floor {
            self.y_max = std::cmp::max(self.y_max, y);
        }
        s.insert(y)
    }

    fn floor(&self) -> u32 {
        self.y_max + 2
    }

    fn lane_hit(&self, p0: Point) -> Option<Point> {
        let (x0, y0) = p0;
        if let Some(lane) = self.lanes.get(&x0) {
            if let Some(y) = lane.iter().filter(|&y1| y0 <= *y1).map(|&y| y).min() {
                return Some((x0, y));
            }
        }
        None
    }

    fn floor_hit(&self, p0: Point) -> Point {
        let (x0, _) = p0;
        if let Some(p) = self.lane_hit(p0) {
            return p;
        }
        (x0, self.floor()) 
    }

    fn contains(&self, p: Point) -> bool {
        let (x, y) = p;
        self.lanes.contains_key(&x) && self.lanes.get(&x).unwrap().contains(&y)
    }

    fn insert_line(&mut self, p0: Point, p1: Point) {
        let (x0, y0) = p0;
        let (x1, y1) = p1;
        for dx in 0..(1 + std::cmp::max(x0, x1) - std::cmp::min(x0, x1)) {
            for dy in 0..(1 + std::cmp::max(y0, y1) - std::cmp::min(y0, y1)) {
                let p = (std::cmp::min(x0, x1) + dx, std::cmp::min(y0, y1) + dy);
                self.insert(p, true);
            }
        }
    }

}

fn parse_pt(ptstr: &str) -> Point {
    let mut parts = ptstr.split(",");
    let x = parts.next().unwrap().parse::<u32>().unwrap();
    let y = parts.next().unwrap().parse::<u32>().unwrap();
    (x, y)
}

fn parse(file: &File) -> std::io::Result<LaneMap> {
    let mut lmap = LaneMap::new();
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let lstr = line?;
        let parts: Vec<&str> = lstr.split(" -> ").collect();
        for i in 0..parts.len() - 1 {
            let p0 = parse_pt(parts[i]);
            let p1 = parse_pt(parts[i + 1]);
            lmap.insert_line(p0, p1);
        }
    }
    Ok(lmap)
}

fn trace_sand_unit(lmap: &LaneMap, p0: Point) -> Option<Point> {
    if let Some((x1, y1)) = lmap.lane_hit(p0) {
        if !lmap.contains((x1 - 1, y1)) {
            return trace_sand_unit(lmap, (x1 - 1, y1));
        } else if !lmap.contains((x1 + 1, y1)) {
            return trace_sand_unit(lmap, (x1 + 1, y1));
        }
        return Some((x1, y1));
    }
    None
}

fn trace_sand_unit2(lmap: &LaneMap, p0: Point) -> Option<Point> {
    if lmap.contains(p0) {
        return None;
    }
    if let Some((x1, y1)) = lmap.lane_hit(p0) {
        if !lmap.contains((x1 - 1, y1)) {
            return trace_sand_unit2(lmap, (x1 - 1, y1));
        } else if !lmap.contains((x1 + 1, y1)) {
            return trace_sand_unit2(lmap, (x1 + 1, y1));
        }
        return Some((x1, y1));
    }
    Some(lmap.floor_hit(p0))
}

fn solve2(file: &File) -> std::io::Result<()> {
    let mut lmap = parse(&file)?;
    let mut cnt = 0;
    while let Some((x, y)) = trace_sand_unit2(&lmap, (500, 0)) {
        lmap.insert((x, y - 1), false);
        cnt += 1;
    }
    println!("solution 2: {} came to rest", cnt);
    Ok(())
}

fn solve1(file: &File) -> std::io::Result<()> {
    let mut lmap = parse(&file)?;
    let mut cnt = 0;
    while let Some((x, y)) = trace_sand_unit(&lmap, (500, 0)) {
        lmap.insert((x, y - 1), false);
        cnt += 1;
    }
    println!("solution 1: {} came to rest", cnt);
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